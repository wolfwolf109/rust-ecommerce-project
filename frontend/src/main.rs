use serde::Deserialize;
use yew::prelude::*;
use gloo_net::http::Request;

#[derive(Clone, PartialEq, Deserialize)]
struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}

#[derive(Properties, PartialEq)]
struct VideosListProps {
    videos: Vec<Video>,
    on_click: Callback<Video>,
}

#[derive(Properties, PartialEq)]
struct VideosDetailsProps {
    video: Video,
}

#[function_component(VideoDetails)]
fn video_details(VideosDetailsProps { video }: &VideosDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ video.title.clone() }</h3>
            <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>
    }
}

#[function_component(VideosList)]
fn videos_list(VideosListProps { videos, on_click}: &VideosListProps) -> Html {
    let on_click = on_click.clone();
    videos.iter().map(|video| {
        let on_video_select = {
            let on_click = on_click.clone();
            let video = video.clone();
            Callback::from(move |_| {
                on_click.emit(video.clone());
            })
        };
        html! {
            <p key={video.id} onclick={on_video_select}>{format!("{}: {}", video.speaker, video.title)}</p>
        }
    }).collect::<Html>()
}


#[function_component(App)]
fn app() -> Html {
    let videos = use_state(|| vec![]);

    {
        let videos = videos.clone();
        use_effect_with((), move |_| {
            let videos = videos.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_videos: Vec<Video> = Request::get("/tutorial/data.json")
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
                videos.set(fetched_videos);
            });
            || ()
        });
    }

    let selected_video = use_state(|| None);

    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| {
            selected_video.set(Some(video));
        })
    };

    let details = selected_video.as_ref().map(|video| {
        html! {
            <VideoDetails video={video.clone()} />
        }
    });

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
           <VideosList videos={(*videos).clone()} on_click={on_video_select.clone()}/>

        <div>
            { for details }
            </div>
        </div>
    };
}

fn main() {
    yew::Renderer::<App>::new().render();
}
