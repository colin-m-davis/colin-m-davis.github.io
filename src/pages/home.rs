use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="container mt-5 px-5">
            <h1 class="title">{ "Home" }</h1>
            <div class="box">
                <h1 class="subtitle block">{ "Hello" }</h1>
                <p class="block">{ "I'm Colin, and this is my personal website." }</p>
                <p class="block">{ "Here's a few things you can do while you're here:" }</p>
                <li><a class="block" href="projects">{ "Check out some of my projects" }</a></li>
            </div>
            <div class="box">
                <h1 class="subtitle block">{ "Contact me" }</h1>
                <li><a class="block" href="mailto: colinmichaelsdavis@gmail.com">{ "colinmichaelsdavis@gmail.com" }</a></li>
                <li><a class="block" href="tel:415-763-9286">{ "415-763-9286" }</a></li>
                <li><a class="block" href="https://github.com/colin-m-davis">{ "GitHub" }</a></li>
                <li><a class="block" href="https://linkedin.com/in/colin-m-davis">{ "LinkedIn" }</a></li> 
            </div>
            <footer>
            {"I built the site in Rust, compiling to WebAssembly."}
            </footer>
        </div>
    }
}