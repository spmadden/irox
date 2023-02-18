use std::{fmt::{Display, Debug, format}, sync::{Arc, Mutex}, collections::BTreeMap};


use pdf::{object::{Catalog, NameDictionary, Resolve, PageRc, Stream, GraphicsStateParameters, MaybeRef, PlainRef}, primitive::{Dictionary, PdfString, Primitive, Name}, PdfError, content::Op, file::File};
use std::collections::HashMap;

use crate::{open, pdf::{IntOp, LayerTree}};
use crate::tabs::tabs;


fn print_if<T: Debug>(val: &Option<T>, level: usize, name: impl Display) {
    if val.is_some() {
        println!("{}{}: {:?}", tabs(level), name, *val);
    }
}



pub fn print_single_prim(level: usize, resolver: &impl Resolve, name: &Name, prim: &Primitive) {
    print!("{}{}={} ", tabs(level), name, prim.get_debug_name());
    match prim {
        Primitive::Array(vec) => {
            println!("{{");
            vec.iter().for_each(|p| print_single_prim(level+1, resolver, name, p));
            println!("{}}}", tabs(level));
        },
        Primitive::Dictionary(dict) => {
            println!("{{");
            print_dictionary(dict, level+1, resolver, name);
            println!("{}}}", tabs(level+1));
        },
        Primitive::Reference(r) => {
            let resolved = resolver.resolve(*r).unwrap();
            println!("({}){{", prim);
            print_single_prim(level+1, resolver, name, &resolved);
            println!("{}}}", tabs(level+1));
        },
        Primitive::String(str) => {
            println!("({})", str.to_string_lossy());
        }
        _ => println!("({})", prim)
    }
}


pub fn print_dictionary(dict: &Dictionary, level: usize, resolver: &impl Resolve, name: impl Display) {
    println!("{}Dictionary {} {{", tabs(level), name);
    dict.iter().for_each(|(name, prim)| {
        print_single_prim(level+1, resolver, name, prim);
    });
    println!("{}}}", tabs(level));
}

pub fn print_graphicsstate(name: &Name, state: &GraphicsStateParameters, level: usize) {
    println!("{}GraphicsState {}{{", tabs(level), name);

    print_if(&state.rendering_intent, level+1, "rendering_intent");
    
    

    println!("{}}}", tabs(level));
}

pub fn print_limits(limits: &Option<(PdfString, PdfString)>, level: usize) {
    match limits {
        Some((start, end)) => println!("{}Limits {{{} -> {}}}", tabs(level), start.to_string_lossy(), end.to_string_lossy()),
        None => println!("{}Limits {{}}", tabs(level))
    }
}

pub fn print_name_dictionary(name: impl Display, level: usize, name_dict: &NameDictionary, resolve: &impl Resolve) {
    println!("{}NameDictionary {} {{", tabs(level), name);
    if let Some(pages) = &name_dict.pages {
        println!("{}Pages {{", tabs(level+1));
        print_limits(&pages.limits, level+2);

        if let Err(err) = pages.walk(resolve, &mut |name, prim| {
            println!("{}{} {{", tabs(level+2), name.to_string_lossy());
            println!("{}{}", tabs(level+3), prim.get_debug_name());
            println!("{}}}", tabs(level+2));
        }) {
            eprintln!("Error walking pdf: {}", err);
        };

        //TODO more.
        println!("{}}}", tabs(level+1));
    }
    if let Some(files) = &name_dict.embedded_files {
        println!("{} EmbeddedFiles {{", tabs(level+1));
        print_limits(&files.limits, level+2);
        if let Err(err) = files.walk(resolve, &mut |name, filespec| {
            println!("{}{} {{", tabs(level+2), name.to_string_lossy());
            println!("{}{:?}", tabs(level+3), filespec);
            println!("{}}}", tabs(level+2));
        }) {
            eprintln!("Error walking pdf: {}", err);
        };
        
        println!("{}}}", tabs(level+1));
    }
    println!("{}}}", tabs(level));
}

pub fn print_catalog(catalog: &Catalog, resolve: &impl Resolve) {
    if let Some(names) = &catalog.names {
        print_name_dictionary("Names",0,  &names, resolve);
    } else {
        println!("NameDictionary Names: {{}}");
    }
    if let Some(dests) = &catalog.dests {
        print_dictionary(dests, 1, resolve, "Dests");
    } else {
        println!("Dictionary Dests: {{}}");
    }
    if let Some(metadata) = &catalog.metadata {
        println!("Metadata {{");
        let res = resolve.resolve(metadata.get_inner());
        print_primitive(res, 1, resolve);
        println!("}}");
    } else {
        println!("Metadata: {{}}");
    }
}

pub fn print_primitive(res: Result<Primitive, PdfError>, level: usize, resolver: &impl Resolve) {
    match res {
        Ok(prim) => {
            match prim {
                Primitive::Stream(stream) => {
                    let str = Stream::<Primitive>::from_stream(stream, resolver).unwrap();
                    println!("Stream {{");
                        println!("{}hasFile: {}", tabs(level+1), &str.file.is_some());
                        print_single_prim(level+1, resolver, &"prim".into(), &str.info);
                    println!("{}}}", tabs(level));

                }
                _ => println!("{}", prim.get_debug_name())
            }
        },
        Err(err) => println!("Err({})", err)
    }
}

pub fn print_page(page: &PageRc, resolver: &impl Resolve)
{
    println!("Page {{");

    if let Some(metadata) = &page.metadata {
        print_single_prim(0, resolver, &"Metadata".into(), metadata);
    }
    if let Some(lgi) = &page.lgi {
        print_single_prim(0, resolver, &"LGIDict".into(), lgi);
    }
    if let Some(vp) = &page.vp {
        print_single_prim(0, resolver, &"VP".into(), vp);
    }

    if let Some(contents) = &page.contents {
        println!("{}Contents {{", tabs(1));
        let ops = contents.operations(resolver).unwrap();
        println!("{}Found {} ops", tabs(2), ops.len());

        let mut stats : HashMap<String, u32> = HashMap::new();
        let mut count: u32 = 0;
        ops.iter().for_each(|op| {
            let key : String = IntOp::from(op).get_debug_name().into();
            stats.entry(key).and_modify(|e| *e += 1).or_insert(0);
            if let Op::BeginMarkedContent{tag, properties} = op {
                println!("{}..{}", tabs(2), count);
                println!("{}BeginMarkedContent(tag={}, props={:?})", tabs(2), tag, properties);
            }
            else if let Op::EndMarkedContent = op {
                println!("{}EndMarkedContent ({})", tabs(2), count);
            }
            else if let Op::XObject { name } = op {
                println!("{}XObject({})", tabs(2), name);
            }
            else {
                count += 1;
            }
        });
        stats.iter().for_each(|(op, val)| println!("{}{} : {}", tabs(2), op, val));

        println!("{}}}", tabs(1));
    }

    if let Some(resources) = &page.resources {
        println!("{}Resources {{", tabs(1));

        {
            println!("{}Patterns {{", tabs(2));
            resources.pattern.iter().for_each(|(name, pat)| {
                println!("{}{}=...", tabs(3), name);
            });
            println!("{}}}", tabs(2));
        }
        {
            println!("{}Properties {{", tabs(2));
            resources.properties.iter().for_each(|(name, dict)| {
                print_dictionary(dict, 3, resolver, name);
            });
            println!("{}}}", tabs(2));
        }
        {
            println!("{}XObjects {{", tabs(2));
            resources.xobjects.iter().for_each(|(name, obj)| {
                print!("{}{} : ", tabs(3), name);
                let res = resolver.resolve(obj.get_inner());
                print_primitive(res, 3, resolver);
            });
            println!("{}}}", tabs(2));
        }
        {
            println!("{}GraphicsStates {{", tabs(2));
            resources.graphics_states.iter().for_each(|(name, state)| {
                print_graphicsstate(name, state, 4);
            });
            println!("{}}}", tabs(2));
        }
        println!("{}}}", tabs(1));
    }
    println!("}}");
}

pub fn print_pdf_info(input_file : &String) {
    
    let pdf_file = open::open(input_file);
    
    let layer_list = get_pdf_layer_list(&pdf_file);
    println!("Found Layers: {{");
    layer_list.layer_set().iter().for_each(|l| println!("{}{}", tabs(1), l));
    println!("}}");
    layer_list.print();


    let num_pages = pdf_file.num_pages();
    println!("Num Pages: {}", num_pages);
    {
        let trailer = &pdf_file.trailer;
        println!("ID: {{");
        trailer.id.iter().for_each(|e| {
            println!("\t{:?}", e.to_string_lossy());
        });
        println!("}}");
        if let Some(info) = &trailer.info_dict {
            print_dictionary(info, 0, &pdf_file, "Info");
        }
        
    }

    pdf_file.pages().for_each(|f| {
        if let Ok(page) = f {
            print_page(&page, &pdf_file);
        }
    });
    
    let catalog = pdf_file.get_root();
    print_catalog(catalog, &pdf_file);
}

pub fn get_pdf_layer_list(pdf_file: &File<Vec<u8>>) -> LayerTree {
    let mut layers : BTreeMap<String, Vec<PlainRef>> = BTreeMap::new();

    pdf_file.pages().for_each(|pagerc| {
        match pagerc {
            Ok(page) => {
                if let Some(resources) = &page.resources {
                    resources.properties.iter()    
                        .for_each(|(name, dict)| {
                            let name_str : String = format!("{}", name);
                            if let Some(Primitive::Name(type_str)) = dict.get("Type") {
                                if type_str == "OCMD" {
                                    if let Some(Primitive::Array(arr)) = dict.get("OCGs") {
                                        layers.insert(name_str, arr.iter().filter_map(|e| {
                                            match e {
                                                Primitive::Reference(r) => Some(*r),
                                                _ => None
                                            }
                                        }).collect());
                                    }
                                }
                            }
                        });
                }
            },
            Err(_) => todo!(),
        }
    });

    let mut tree = LayerTree::new();

    layers.iter().for_each(|(k, v)| {
        let names : Vec<String> = v.iter().filter_map(|r| {
            if let Ok(Primitive::Dictionary(resolved)) = pdf_file.resolve(*r) {
                if let Some(Primitive::String(name)) = resolved.get("Name") {
                    return Some(name.to_string_lossy());
                }
            }
            None
        }).collect();
        tree.add(names, k.clone());
    });
    
    tree
}