use crates_caching_proxy_libs::{config, git};
use git2::{RepositoryInitOptions, FetchOptions, ErrorCode, Signature, Commit};



#[test]
pub fn git_download() -> Result<(), ()> {
    let repo_path = "test-repo";
    let repo = match git2::Repository::open_bare(repo_path) {
        Ok(repo) => {
            println!("Found repo at: {repo_path}");
            repo
        },
        Err(e) => {
            if e.code() == ErrorCode::NotFound {
                let mut opts = RepositoryInitOptions::new();
                opts.bare(true)
                    .origin_url(config::DEFAULT_CRATESIO_UPSTREAM);
                let repo = match git2::Repository::init_opts(repo_path, &opts) {
                    Ok(repo) => repo,
                    Err(e) => panic!("{:?}", e)
                };
                repo
            } else {
                panic!("{:?}", e);
            }
        }
    };

    let mut origin = repo.find_remote("origin").expect("Missing origin.");

    let refspecs = ["refs/heads/master:refs/remotes/origin/master"];
    let reflog_msg = "fetched.";
    let mut fopts = FetchOptions::new();
    
    fopts.remote_callbacks(git::stdout_callbacks());

    if let Err(e) = origin.fetch(&refspecs, Some(&mut fopts), Some(reflog_msg)) {
        panic!("{:?}", e);
    };
    
    let rem_master = match repo.find_branch("origin/master", git2::BranchType::Remote) {
        Ok(br) => {
            println!("Found branch {}", br.name().expect("").unwrap());
            br
        },
        Err(e) => panic!("{:?}", e),
    };
    let rem_master_ref = rem_master.into_reference();
    let tree = match rem_master_ref.peel_to_tree() {
        Ok(tree) => {
            tree
        },
        Err(e) =>  panic!("{:?}", e),
    };

    let mut treebuilder = match repo.treebuilder(Some(&tree)) {
        Ok(tb) => tb,
        Err(e) => panic!("{:?}", e)
    };

    let cnfjson = "{\"dl:\":\"http://localhost:8000/dl/\",\"api\":\"http://localhost:8000/api/\"}";
    let cnfoid = repo.odb().expect("").write(git2::ObjectType::Blob, &cnfjson.as_bytes()).expect("");

    {
        let _res = treebuilder.insert("config.json", cnfoid, 0o100644).expect("");
    }
    let res = treebuilder.write().expect("");
    let new_tree = repo.find_tree(res).expect("msg");

    let rem_master_cmit = rem_master_ref.peel_to_commit().expect("msg");

    
    let master = match repo.find_branch("master", git2::BranchType::Local) {
        Ok(master) => {
            Some(master.get().peel_to_commit().expect("commit"))
        },
        Err(e) => {
            None
        }
    };

    let author = Signature::now("auto-name", "auto@email.now").expect("author");
    let committer = &author;

    let update_ref = Some("refs/heads/master");
    let message = format!("Update from {}", rem_master_cmit.id());
    let new_master = match master {
        Some(m) => {
            repo.commit(update_ref, &author, committer, message.as_str(), &new_tree, &[&m]).expect("commit")
        }, 
        None => {
            repo.commit(update_ref, &author, committer, message.as_str(), &new_tree, &[]).expect("commit")
        }
    };
    println!("Master is now at: {}", new_master);

    Ok(())
}