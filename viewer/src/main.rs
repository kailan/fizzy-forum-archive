use fastly::{
    http::StatusCode,
    Error, Request, Response,
};
use include_dir::{include_dir, Dir, DirEntry};
use structure::{ExportMetadata, Post, ThreadMetadata};

mod templates;

static CONTENT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../crawler/out");

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    let resp = match req.get_path() {
        "/" => Response::from_body(templates::render_index_page(&get_export_metadata())),
        _ if req.get_path().starts_with("/forum/") && req.get_path().contains("/thread/") => {
            let mut split = req.get_path().split('/');
            let forum_id = split.nth(2).unwrap();
            let thread_id = split.nth(1).unwrap();

            // Get forum metadata
            let mut forums = get_export_metadata().forums;

            let forum = match forums.iter().position(|f| f.id == forum_id) {
                Some(forum_pos) => forums.swap_remove(forum_pos),
                None => {
                    return Ok(Response::from_body(templates::render_error_page(
                        "Forum not found",
                    ))
                    .with_status(StatusCode::NOT_FOUND));
                }
            };

            // Get thread metadata
            let mut threads = get_threads_for_forum(forum_id);

            let thread = match threads.iter().position(|t| t.id == thread_id) {
                Some(thread_pos) => threads.swap_remove(thread_pos),
                None => {
                    return Ok(Response::from_body(templates::render_error_page(
                        "Thread not found",
                    ))
                    .with_status(StatusCode::NOT_FOUND));
                }
            };

            let mut posts: Vec<Post> = CONTENT_DIR
                .find(&format!("{}/{}/*.json", forum_id, thread_id))
                .unwrap()
                .map(|f| match f {
                    DirEntry::File(f) => {
                        serde_json::from_str(f.contents_utf8().unwrap()).unwrap()
                    }
                    _ => panic!("Unexpected directory entry"),
                })
                .collect();

            posts.sort_by(|a, b| a.created_date.cmp(&b.created_date));

            Response::from_body(templates::render_thread_page(forum, thread, posts))
        }
        _ if req.get_path().starts_with("/forum/") => {
            let id = req.get_path().split('/').nth(2).unwrap();

            let mut forums = get_export_metadata().forums;

            let forum = match forums.iter().position(|f| f.id == id) {
                Some(forum_pos) => forums.swap_remove(forum_pos),
                None => {
                    return Ok(Response::from_body(templates::render_error_page(
                        "Forum not found",
                    ))
                    .with_status(StatusCode::NOT_FOUND));
                }
            };

            let threads: Vec<ThreadMetadata> = serde_json::from_str(
                CONTENT_DIR
                    .get_file(format!("{}/threads.json", forum.id))
                    .unwrap()
                    .contents_utf8()
                    .unwrap(),
            )
            .unwrap();

            Response::from_body(templates::render_forum_page(forum, threads))
        }
        "/style.css" => Response::from_body(include_str!("templates/style.css")),
        _ => Response::from_body(templates::render_error_page("Page not found"))
            .with_status(StatusCode::NOT_FOUND),
    };

    Ok(resp)
}

fn get_export_metadata() -> ExportMetadata {
    serde_json::from_str(
        CONTENT_DIR
            .get_file("export.json")
            .unwrap()
            .contents_utf8()
            .unwrap(),
    )
    .expect("failed to parse export metadata")
}

fn get_threads_for_forum(forum_id: &str) -> Vec<ThreadMetadata> {
    serde_json::from_str(
        CONTENT_DIR
            .get_file(format!("{}/threads.json", forum_id))
            .unwrap()
            .contents_utf8()
            .unwrap(),
    )
    .unwrap()
}
