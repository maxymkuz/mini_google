#![recursion_limit = "1024"]

use std::net::{TcpStream};
use std::io::{Read, Write};
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use std::str::from_utf8;


struct Model {
    link: ComponentLink<Self>,
}

enum Msg {}

impl Component for Model {
    type Message = ();
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <body>
            <header>
                <div class="section">
                    <h1 class="title">{"Mini Google"}</h1>
                    <h2 class="subtitle">{"Experimental Search Engine"}</h2>
                </div>
            </header>

            <section class="section">
                <div class="columns">
                    <div class="column is-one-quarter">
                        <h2 class="subtitle">{"Library"}</h2>
                        <ul>
                            <li><a href="">
                                <div class="columns">
                                    <div class="column is-1">
                                        <i class="fa fa-eye">
                                        </i>
                                    </div>
                                    <div class="column">
                                        {"Search History"}
                                    </div>
                                </div>
                            </a></li>
                            <li><a href="">
                                <div class="columns">
                                    <div class="column is-1">
                                        <i class="fa fa-bookmark">
                                        </i>
                                    </div>
                                    <div class="column">
                                        {"Bookmarks"}
                                    </div>
                                </div>
                            </a></li>
                        </ul>
                    </div>
                    <div class="column">
                        <div class="container">
                            <div class="field">
                                <div class="columns">
                                    <div class="column is-1">
                        <span class="icon is-large">
                            <i class="fa fa-meh-o"></i>
                        </span>
                                    </div>
                                    <div class="column">
                                        <form action="http://0.0.0.0:3333" method="get">
                                            <div class="field has-addons">
                                                <p class="control is-expanded">
                                                    <input class="input is-link" type="text"
                                                    placeholder="Search" name="user_search"></input>
                                                </p>
                                                <div class="control">
                                                    <button type="submit" class="button is-link">
                                                        <i class="fa fa-search"></i>
                                                    </button>
                                                </div>
                                            </div>
                                        </form>
                                    </div>
                                    <div class="column is-3">
                                        <div class="dropdown is-hoverable">
                                            <div class="dropdown-trigger">
                                                <button class="button" aria-haspopup="true" aria-controls="dropdown-menu">
                                                    <span>{"Search Language"}</span>
                                                    <span class="icon is-small">
                                    <i class="fa fa-angle-down" aria-hidden="true"></i>
                                    </span>
                                                </button>
                                            </div>
                                            <div class="dropdown-menu" id="dropdown-menu" role="menu">
                                                <div class="dropdown-content">
                                                    <a href="#" class="dropdown-item is-active">
                                                        {"English"}
                                                    </a>
                                                    <a class="dropdown-item">
                                                        {"Українська"}
                                                    </a>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                                <ul class="nostyle home">
                                            <li class="section">
                                                <div class="card">
                                                    <a href="{{ page.permalink | safe }}">
                                                        <div class="card-content">
                                                            <h2 class="title is-4"> {"Title"}</h2>
                                                            <p class="subtitle">{"Path"}</p>
                                                            <p class="subtitle">{"Description"}</p>
                                                        </div>
                                                    </a>
                                                </div>
                                            </li>
                                    <div style="text-align: center">
                                        <h2>
                                            <span class="inactive"><i class="fa fa-arrow-left"></i></span>
                                            {"1 page"}
                                                <a class="active" href="{{ paginator.next }}"><i class="fa fa-arrow-right"></i></a>
                                        </h2>
                                    </div>
                                </ul>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
            <footer class="footer">
                <div class="content has-text-centered">
                    <p>{"Project for Architecture of Computer Systems Course."}</p>
                    <a href="https://github.com/maxymkuz/mini_google"><i
                            class="fa fa-github-alt"></i></a>
                </div>
            </footer>
            </body>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}