mod generated;
use graphql_client::{reqwest::post_graphql, Response};
use yew::prelude::*;

type UserResponseData = generated::query::get_user::ResponseData;
type UsersResponseData = generated::query::get_users::ResponseData;

async fn fetch_user() -> Result<Response<UserResponseData>, reqwest::Error> {
    let client = reqwest::Client::new();
    let url = "http://localhost:4000/graphql";
    let variables = generated::query::get_user::Variables { id: 1 };

    let response = post_graphql::<generated::query::GetUser, _>(&client, url, variables).await?;

    Ok(response)
}

async fn fetch_users() -> Result<Response<UsersResponseData>, reqwest::Error> {
    let client = reqwest::Client::new();
    let url = "http://localhost:4000/graphql";

    let response = post_graphql::<generated::query::GetUsers, _>(
        &client,
        url,
        generated::query::get_users::Variables,
    )
    .await?;

    Ok(response)
}

#[function_component(App)]
fn app() -> Html {
    let users: UseStateHandle<Vec<generated::query::get_users::GetUsersUsers>> =
        use_state(|| vec![]);

    log::info!("{:#?}", users);

    {
        let data = users.clone();
        use_effect_with_deps(
            move |_| {
                let future = async move {
                    match fetch_users().await {
                        Ok(response) => {
                            data.set(response.data.unwrap().users);
                        }
                        Err(err) => log::error!("Could not fetch data. error: {:?}", err),
                    }
                };
                wasm_bindgen_futures::spawn_local(future);

                || ()
            },
            (),
        );
    };

    html! {
        <>
            <h1>{ "GraphQL Conf 2023" }</h1>
            {
                users.iter().map(|e| {
                            html! {
                                <h3>{"Id: "} {e.id} {" // "} {"name: "} { &e.name }</h3>
                            }
                        })
                        .collect::<Html>()
            }
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
