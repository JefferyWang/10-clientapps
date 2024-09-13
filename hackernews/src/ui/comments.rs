#![allow(non_snake_case)]

use crate::Comment;
use dioxus::prelude::*;

use super::CommentsState;

#[component]
pub fn Comments() -> Element {
    let comments_state = use_context::<Signal<CommentsState>>();
    match comments_state() {
        CommentsState::Unset => rsx! {
            div { class: "mt-6",
                p { "Select a story to view comments" }
            }
        },
        CommentsState::Loading => rsx! {
            div { class: "mt-6",
                p { "Loading comments..." }
            }
        },
        CommentsState::Loaded(data) => rsx! {
            // use StoryComment
            ul {
                for comment in data.comments {
                    StoryComment { comment: comment.clone() }
                }
            }
        },
    }
}

#[component]
pub fn StoryComment(comment: Comment) -> Element {
    rsx! {
        li {
            article { class: "p-4 leading-7 tracking-wider text-gray-500 border-b border-gray-200",
                span { "{comment.by} {comment.time} | next [-]" }
                div {
                    dangerous_inner_html: "{comment.text}"
                }
            }
        }
    }
}
