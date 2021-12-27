use std::fmt::Write;

use serde::Serialize;
use structure::{ExportMetadata, ForumMetadata, Post, ThreadMetadata};
use tinytemplate::TinyTemplate;

#[derive(Serialize)]
struct ForumViewContext {
    pub forum: ForumMetadata,
    pub threads: Vec<ThreadMetadata>,
}

#[derive(Serialize)]
struct ThreadViewContext {
    pub forum: ForumMetadata,
    pub thread: ThreadMetadata,
    pub posts: Vec<Post>,
}

#[derive(Serialize)]
struct ErrorContext<'a> {
    pub message: &'a str,
}

pub fn render_index_page(export: &mut ExportMetadata) -> String {
    let renderer = {
        let mut tt = TinyTemplate::new();
        tt.add_template("index", include_str!("index.html"))
            .expect("failed to load index template");
        tt
    };

    export.forums.retain(|f| f.post_count > 0);

    renderer.render("index", &export).expect("render failed")
}

pub fn render_forum_page(forum: ForumMetadata, threads: Vec<ThreadMetadata>) -> String {
    let renderer = {
        let mut tt = TinyTemplate::new();
        tt.add_template("topics", include_str!("topics.html"))
            .expect("failed to load topics template");
        tt
    };

    renderer
        .render("topics", &ForumViewContext { forum, threads })
        .expect("render failed")
}

pub fn render_thread_page(
    forum: ForumMetadata,
    thread: ThreadMetadata,
    posts: Vec<Post>,
) -> String {
    let renderer = {
        let mut tt = TinyTemplate::new();
        tt.add_template("thread", include_str!("thread.html"))
            .expect("failed to load thread template");
        tt.add_formatter("vec_counter", |value, output| {
            match value {
                serde_json::Value::Array(array) => write!(output, "{}", array.len()),
                _ => write!(output, "0"),
            }?;
            Ok(())
        });
        tt
    };

    renderer
        .render(
            "thread",
            &ThreadViewContext {
                forum,
                thread,
                posts,
            },
        )
        .expect("render failed")
}

pub fn render_error_page(message: &str) -> String {
    let renderer = {
        let mut tt = TinyTemplate::new();
        tt.add_template("error", include_str!("error.html"))
            .expect("failed to load error template");
        tt
    };

    renderer
        .render("error", &ErrorContext { message })
        .expect("render failed")
}
