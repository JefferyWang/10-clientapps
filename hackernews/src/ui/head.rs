use dioxus::prelude::*;

#[component]
pub fn Head() -> Element {
    rsx! {
        header {
            nav { class: "bg-white border-gray-200 px-4 lg:px-6 py-2.5 dark:bg-gray-800",
                div { class: "flex flex-wrap justify-between items-center mx-auto max-w-screen-xl",
                    a { href: "https://flowbite.com", class: "flex items-center",
                        img {
                            src: "https://flowbite.com/docs/images/logo.svg",
                            alt: "Flowbite Logo",
                            class: "mr-3 h-6 sm:h-9"
                        }
                        span { class: "self-center text-xl font-semibold whitespace-nowrap dark:text-white",
                            "Flowbite"
                        }
                    }
                    div { class: "flex items-center lg:order-2",
                        a {
                            href: "#",
                            class: "text-gray-800 dark:text-white hover:bg-gray-50 focus:ring-4 focus:ring-gray-300 font-medium rounded-lg text-sm px-4 lg:px-5 py-2 lg:py-2.5 mr-2 dark:hover:bg-gray-700 focus:outline-none dark:focus:ring-gray-800",
                            "Log in"
                        }
                        a {
                            href: "#",
                            class: "text-white bg-primary-700 hover:bg-primary-800 focus:ring-4 focus:ring-primary-300 font-medium rounded-lg text-sm px-4 lg:px-5 py-2 lg:py-2.5 mr-2 dark:bg-primary-600 dark:hover:bg-primary-700 focus:outline-none dark:focus:ring-primary-800",
                            "Get started"
                        }
                        button {
                            "data-collapse-toggle": "mobile-menu-2",
                            "aria-expanded": "false",
                            "aria-controls": "mobile-menu-2",
                            r#type: "button",
                            class: "inline-flex items-center p-2 ml-1 text-sm text-gray-500 rounded-lg lg:hidden hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600",
                            span { class: "sr-only", "Open main menu" }
                            svg {
                                "fill": "currentColor",
                                "xmlns": "http://www.w3.org/2000/svg",
                                "viewBox": "0 0 20 20",
                                class: "w-6 h-6",
                                path {
                                    "clip-rule": "evenodd",
                                    "fill-rule": "evenodd",
                                    "d": "M3 5a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 10a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 15a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z"
                                }
                            }
                            svg {
                                "xmlns": "http://www.w3.org/2000/svg",
                                "fill": "currentColor",
                                "viewBox": "0 0 20 20",
                                class: "hidden w-6 h-6",
                                path {
                                    "fill-rule": "evenodd",
                                    "d": "M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z",
                                    "clip-rule": "evenodd"
                                }
                            }
                        }
                    }
                    div {
                        class: "hidden justify-between items-center w-full lg:flex lg:w-auto lg:order-1",
                        id: "mobile-menu-2",
                        ul { class: "flex flex-col mt-4 font-medium lg:flex-row lg:space-x-8 lg:mt-0",
                            li {
                                a {
                                    href: "#",
                                    "aria-current": "page",
                                    class: "block py-2 pr-4 pl-3 text-white rounded bg-primary-700 lg:bg-transparent lg:text-primary-700 lg:p-0 dark:text-white",
                                    "Home"
                                }
                            }
                            li {
                                a {
                                    href: "#",
                                    class: "block py-2 pr-4 pl-3 text-gray-700 border-b border-gray-100 hover:bg-gray-50 lg:hover:bg-transparent lg:border-0 lg:hover:text-primary-700 lg:p-0 dark:text-gray-400 lg:dark:hover:text-white dark:hover:bg-gray-700 dark:hover:text-white lg:dark:hover:bg-transparent dark:border-gray-700",
                                    "Company"
                                }
                            }
                            li {
                                a {
                                    href: "#",
                                    class: "block py-2 pr-4 pl-3 text-gray-700 border-b border-gray-100 hover:bg-gray-50 lg:hover:bg-transparent lg:border-0 lg:hover:text-primary-700 lg:p-0 dark:text-gray-400 lg:dark:hover:text-white dark:hover:bg-gray-700 dark:hover:text-white lg:dark:hover:bg-transparent dark:border-gray-700",
                                    "Marketplace"
                                }
                            }
                            li {
                                a {
                                    href: "#",
                                    class: "block py-2 pr-4 pl-3 text-gray-700 border-b border-gray-100 hover:bg-gray-50 lg:hover:bg-transparent lg:border-0 lg:hover:text-primary-700 lg:p-0 dark:text-gray-400 lg:dark:hover:text-white dark:hover:bg-gray-700 dark:hover:text-white lg:dark:hover:bg-transparent dark:border-gray-700",
                                    "Features"
                                }
                            }
                            li {
                                a {
                                    href: "#",
                                    class: "block py-2 pr-4 pl-3 text-gray-700 border-b border-gray-100 hover:bg-gray-50 lg:hover:bg-transparent lg:border-0 lg:hover:text-primary-700 lg:p-0 dark:text-gray-400 lg:dark:hover:text-white dark:hover:bg-gray-700 dark:hover:text-white lg:dark:hover:bg-transparent dark:border-gray-700",
                                    "Team"
                                }
                            }
                            li {
                                a {
                                    href: "#",
                                    class: "block py-2 pr-4 pl-3 text-gray-700 border-b border-gray-100 hover:bg-gray-50 lg:hover:bg-transparent lg:border-0 lg:hover:text-primary-700 lg:p-0 dark:text-gray-400 lg:dark:hover:text-white dark:hover:bg-gray-700 dark:hover:text-white lg:dark:hover:bg-transparent dark:border-gray-700",
                                    "Contact"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
