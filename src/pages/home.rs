use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="container mt-5 px-5">
            <div class="tile is-parent is-vertical">
                <h1 class="title">{ "Hello :-)" }</h1>
                <div class="block tile is-horizontal">
                    <div class="tile is-child is-6">
                        <h1 class="subtitle block">{ "About me" }</h1>
                        <p class="block">{ "I'm Colin. I live in California, USA. In my free time I study math & computers, appreciate nature,
                        and listen to music. Basically, I feel most at home when I'm solving technical problems in an outdoor setting with my headphones on." }</p>
                        <p class="block">{ "I learn best by working with stuff other people have made, taking a look at what's going on under the hood, then implementing my own ideas.
                        If you're interested in collaborating with me or have questions about my work, don't be afraid to reach out - I love discussing new ideas." }</p>
                        <a class="block" href="projects">{ "Check out some of my projects!" }</a>
                    </div>
                    <div class="tile is-child is-6 py-5 px-5">
                        <figure class="image is-2by1">
                            <img class="block" alt="You can't remember your name" src="https://source.unsplash.com/random/800x400/?desert" />
                        </figure>
                    </div>
                </div>
            </div>
            <div class="tile is-parent is-vertical">
                <h1 class="subtitle block">{ "Contact info" }</h1>
                <li><a class="block" href="mailto: colinmichaelsdavis@gmail.com">{ "colinmichaelsdavis@gmail.com" }</a></li>
                <li><a class="block" href="tel:415-763-9286">{ "415-763-9286" }</a></li>
                <li><a class="block" href="https://github.com/colin-m-davis">{ "GitHub" }</a></li>
                <li><a class="block" href="https://linkedin.com/in/colin-m-davis">{ "LinkedIn" }</a></li>
            </div>
        </div>
    }
}
