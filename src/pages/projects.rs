use yew::prelude::*;

#[function_component(Projects)]
pub fn projects() -> Html {

    let archive = ProjectProps {
        name: String::from("Archive"),
        tech: String::from("C"),
        link: String::from("https://github.com/colin-m-davis/Archive"),
        desc: String::from("A version control system"),
    };

    html! {
        <div class="container mt-5 px-5">
            <h1 class="title">{ "Projects" }</h1>
            <p class="block">{ "Hey, here are some of the projects I work on in my spare time! Each project title is a hyperlink to the corresponding GitHub repository." }</p>
            <div class="box">
                <h1 class="subtitle block"><a href="https://github.com/colin-m-davis/Archive">{ "Archive" }</a></h1>
                <p class="block">{ "In an effort to understand the intricacies of Git, I decided to make my own version control system, Archive, written in C.
                Archive uses zlib to compress data. Users can Document their project in an Archive, creating a merkle tree representation of the project.
                Each leaf in this tree is a SHA-256 hash corresponding to a file. The compressed version of this file is located within the project's Archive in a directory matching this hash.
                I plan to incorporate an advanced merge feature that analyzes potential conflicts to faciliate collaboration between peers." }</p>
            </div>
            <div class="box">
                <h1 class="subtitle block"><a href="https://github.com/colin-m-davis/pathfinder">{ "Pathfinder" }</a></h1>
                <p class="block">{ "Pathfinder is a simple command-line interface tool that I built in C++ that helps the user traverse and manipulate a Unix filesystem. Nothing crazy here.
                I used basic graph traversal algorithms like breath-first search to perform these operations. Check out the ReadMe on Github for more information on the commands."}</p>
            </div>
            <div class="box">
                <h1 class="subtitle block"><a href="https://github.com/colin-m-davis/Span">{ "Span" }</a></h1>
                <p class="block">{ "Span is my take on a linear algebra library. I acknowledge that the speed of my C++ code may not rival that of expertly-crafted libraries like Boost or NumPy,
                however, raw computational efficiency was not the focus of this project. Instead, I decided to write an interface that encourages users to adopt a theoretical mindset. In this way, the user is not limited to operations involving basic scalar types like integers or floats.
                For example, the Span::Matrix data structure accomodates any type that adheres to the field axioms. I aim to widen the pool of matrix and vector operations available in Span and flesh out the documentation." }</p>
            </div>
            <div class="box">
                <h1 class="subtitle block"><a href="https://github.com/colin-m-davis/Runner">{ "Runner" }</a></h1>
                <p class="block">{ "Runner is a valet ticket management app. In my time as a valet, I have encountered ticket management systems that have a clunky UI and are slow to refresh.
                Guests don't have time to wait on an outdated app when they have an appointment to attend! Developing this app was an opportunity for me to learn about different design patterns.
                I chose the model-view-viewmodel (MVVM) pattern in the hopes that my future self or any potential collaborators would have an easier time expanding upon my work. Runner quickly stores and retrieves updates to a Cloud Firestore database via the Firebase API for Swift. Here's a link to a brief video demo of the app."}</p>
                <a href="https://www.youtube.com/shorts/1RsJjclEmOw">{ "https://www.youtube.com/shorts/1RsJjclEmOw" }</a>
            </div>
        </div>

    }
}

#[derive(Properties, PartialEq)]
struct ProjectProps {
    pub name: String,
    pub tech: String,
    pub link: String,
    pub desc: String
}

#[function_component(Project)]
fn project(props: &ProjectProps) -> Html {
    let names = vec![&props.tech];

    html! {
        <div>
            <h1 class="subtitle">{ &props.name }</h1>
            <p>{ &props.tech }</p>
            <p>{ &props.link }</p>
            <p>{ &props.desc }</p>
        </div>
    }
}