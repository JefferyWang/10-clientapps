use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "bg-gray-900 text-gray-300 py-6 px-16 font-sans tracking-wide",
            div { class: "flex justify-between items-center max-lg:flex-col text-center flex-wrap gap-4",
                p { class: "text-[15px] leading-loose", "© ReadymadeUI. All rights reserved." }
                ul { class: "flex space-x-6 gap-y-2 max-lg:justify-center flex-wrap",
                    li {
                        a {
                            href: "javascript:void(0)",
                            class: "text-[15px] hover:text-white",
                            "Terms of Service"
                        }
                    }
                    li {
                        a {
                            href: "javascript:void(0)",
                            class: "text-[15px] hover:text-white",
                            "Privacy Policy"
                        }
                    }
                    li {
                        a {
                            href: "javascript:void(0)",
                            class: "text-[15px] hover:text-white",
                            "Contact"
                        }
                    }
                }
            }
        }
    }
}
