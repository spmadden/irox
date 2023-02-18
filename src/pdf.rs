use std::collections::BTreeMap;
use pdf::content::Op;

use crate::tabs::tabs;

pub struct IntOp<'a>(&'a Op);

impl <'a> From<&'a Op> for IntOp<'a> {
    fn from(value: &'a Op) -> Self {
        return IntOp(&value);
    }
}

impl IntOp<'_> {
    pub fn get_debug_name(&self) -> &str{
        match self.0 {
            Op::BeginMarkedContent { tag:_, properties:_ } => "BeginMarkedContent",
            Op::EndMarkedContent => "EndMarkedContent",
            Op::MarkedContentPoint { tag:_, properties:_ } => "MarkedContentPoint",
            Op::Close => "Close",
            Op::MoveTo { p:_ } => "MoveTo",
            Op::LineTo { p:_ } => "LineTo",
            Op::CurveTo { c1:_, c2:_, p:_ } => "CurveTo",
            Op::Rect { rect:_ } => "Rect",
            Op::EndPath => "EndPath",
            Op::Stroke => "Stroke",
            Op::FillAndStroke { winding:_ } => "FillAndStroke",
            Op::Fill { winding:_ } => "Fill",
            Op::Shade { name:_ } => "Shade",
            Op::Clip { winding:_ } => "Clip",
            Op::Save => "Save",
            Op::Restore => "Restore",
            Op::Transform { matrix:_ } => "Transform",
            Op::LineWidth { width:_ } => "LineWidth",
            Op::Dash { pattern:_, phase:_ } => "Dash",
            Op::LineJoin { join:_ } => "LineJoin",
            Op::LineCap { cap:_ } => "LineCap",
            Op::MiterLimit { limit:_ } => "MiterLimit",
            Op::Flatness { tolerance:_ } => "Flatness",
            Op::GraphicsState { name:_ } => "GraphicsState",
            Op::StrokeColor { color:_ } => "StrokeColor",
            Op::FillColor { color:_ } => "FillColor",
            Op::FillColorSpace { name:_ } => "FillColorSpace",
            Op::StrokeColorSpace { name:_ } => "StrokeColorSpace",
            Op::RenderingIntent { intent:_ } => "RenderingIntent",
            Op::BeginText => "BeginText",
            Op::EndText => "EndText",
            Op::CharSpacing { char_space:_ } => "CharSpacing",
            Op::WordSpacing { word_space:_ } => "WordSpacing",
            Op::TextScaling { horiz_scale:_ } => "TextScaling",
            Op::Leading { leading:_ } => "Leading",
            Op::TextFont { name:_, size:_ } => "TextFont",
            Op::TextRenderMode { mode:_ } => "TextRendererMode",
            Op::TextRise { rise:_ } => "TextRise",
            Op::MoveTextPosition { translation:_ } => "MoveTextPosition",
            Op::SetTextMatrix { matrix:_ } => "SetTextMatrix",
            Op::TextNewline => "TextNewLine",
            Op::TextDraw { text:_ } => "TextDraw",
            Op::TextDrawAdjusted { array:_ } => "TextDrawAdjusted",
            Op::XObject { name:_ } => "XObject",
            Op::InlineImage { image:_ } => "InlineImage",
        }
    }
}


pub struct LayerTree {
    name: String, 
    children: BTreeMap<String, LayerTree>,
    refs: Vec<String>
}

impl LayerTree {
    pub fn new() -> LayerTree {
        Self::new_node(String::from(""))
    }
    pub fn new_node(name: String) -> LayerTree {
        LayerTree { name: name, children: BTreeMap::new(), refs: Vec::new() }
    }

    fn make_name(&self, name: &String) -> String {
        if self.name.len() > 0 {
            return format!("{}.{}", self.name, name);
        }
        return format!("{}", name);
    }

    pub fn add(&mut self, names_in: Vec<String>, reference: String) {
        let mut names : Vec<String> = names_in.clone();

        if names.len() <= 0 {
            return;
        }
        if names.len() == 1 {
            // last in the tree, add the thing.
            let name = names.pop().unwrap();
            if let Some(tree) = self.children.get_mut(&name) {
                tree.refs.push(reference);
            } else {
                // didn't exist, make one.
                let new_name = self.make_name(&name);
                let mut node = Self::new_node(new_name);
                node.refs.push(reference);
                self.children.insert(name, node);
            }
            return;
        }

        let name = names.pop().unwrap();
        if !self.children.contains_key(&name) {
            let new_name = self.make_name(&name);
            let newnode = Self::new_node(new_name.clone());
            self.children.insert(name.clone(), newnode);
        }
        
        if let Some(tree) = self.children.get_mut(&name) {
            tree.add(names, reference);
        }
        
    }

    fn print_inner(&self, indent: usize) {
        println!("{}{} {{", tabs(indent), self.name);
        println!("{}refs: {:?}", tabs(indent+1), self.refs);
        println!("{}children: {{", tabs(indent+1));
        self.children.iter().for_each(|(k,v)| {
            v.print_inner(indent+2);
        });
        println!("{}}}", tabs(indent+1));


        println!("{}}}", tabs(indent));
    }
    pub fn print(&self) {
        self.print_inner(0);
    }

    fn layer_set_inner(&self, out: &mut Vec<String>) {
        if self.children.is_empty() {
            out.push(self.name.clone());
        }
        self.children.iter().for_each(|(_k, v)| {
            v.layer_set_inner(out);
        });
    }
    pub fn layer_set(&self) -> Vec<String> {
        let mut out : Vec<String> = Vec::new();
        self.children.iter().for_each(|(_k, v)| {
            v.layer_set_inner(&mut out);
        });
        
        out
    }
}

