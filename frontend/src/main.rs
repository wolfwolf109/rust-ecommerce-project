use yew::prelude::*;

struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}

#[function_component(App)]
fn app() -> Html {
    let videos = vec![
    Video {
        id: 1,
        title: "Building and breaking things".to_string(),
        speaker: "John Doe".to_string(),
        url: "https://youtu.be/PsaFVLr8t4E".to_string(),
    },
    Video {
        id: 2,
        title: "The development process".to_string(),
        speaker: "Jane Smith".to_string(),
        url: "https://youtu.be/PsaFVLr8t4E".to_string(),
    },
    Video {
        id: 3,
        title: "The Web 7.0".to_string(),
        speaker: "Matt Miller".to_string(),
        url: "https://youtu.be/PsaFVLr8t4E".to_string(),
    },
    Video {
        id: 4,
        title: "Mouseless development".to_string(),
        speaker: "Tom Jerry".to_string(),
        url: "https://youtu.be/PsaFVLr8t4E".to_string(),
    },
];

    let videos = videos
        .iter()
        .map(|video| {
            html! {
                <p key={video.id}>{format!("{}: {}", video.speaker, video.title)}</p>
            }
        })
        .collect::<Html>();

    return html! {
        <div>
            <nav>
                <ul>
                    <li><a href="#">{"Home"}</a></li>
                    <li><a href="#">{"About"}</a></li>
                    <li><a href="#">{"Contact"}</a></li>
                </ul>
            </nav>
            <h3>{ "Videos to watch" }</h3>
           { videos }
        </div>
    };
}

fn main() {
    yew::Renderer::<App>::new().render();
}
