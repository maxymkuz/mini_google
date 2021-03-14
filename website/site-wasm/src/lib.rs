#![recursion_limit="1024"]
use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

enum Msg {
    AddOne,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1
        }
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
                                        <form action="http://localhost:5001" method="post">
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
            </body>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}