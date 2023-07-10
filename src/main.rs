use leptos::*;
use leptos_router::{ActionForm, Route, Router, Routes, A};

#[cfg(feature = "server")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
	use std::{env, sync::mpsc, thread};

	use actix_files::Files;
	use actix_web::{App, HttpServer};
	use leptos_actix::LeptosRoutes;

	env::set_var("LEPTOS_SITE_PKG_DIR", "wasm");

	let leptos_options = leptos::get_configuration(None).await.unwrap().leptos_options;
	let server = HttpServer::new(move || {
		App::new()
			.route("/api/{tail:.*}", leptos_actix::handle_server_fns())
			.leptos_routes(
				leptos_options.to_owned(),
				leptos_actix::generate_route_list(|cx| view! { cx, <App /> }),
				|cx| view! { cx, <App /> },
			)
			.service(Files::new("/", "./static").prefer_utf8(true).redirect_to_slash_directory())
	})
	.bind("localhost:3000")?
	.run();

	// To shutdown the server with Ctrl + C
	let (_tx, rx) = mpsc::channel::<()>();
	let srv = server.handle();
	thread::spawn(move || {
		rx.recv().unwrap_or_default();
		println!("Shutting down!");
		srv.stop(true)
	});

	println!("Listening on http://localhost:3000");
	server.await
}

#[cfg(feature = "client")]
pub fn main() {
	use wasm_bindgen::prelude::wasm_bindgen;

	#[wasm_bindgen]
	pub fn hydrate() {
		leptos::mount_to_body(move |cx| view! { cx, <App /> });
	}
}

#[component]
fn App(cx: Scope) -> impl IntoView {
	view! { cx,
		<Router>
			<Routes>
				<Route path="/" view=|cx| view! { cx,
					<A href="/contact">"Contract"</A>
				} />
				<Route path="/contact" view=|cx| view! { cx, <Contract /> } />
			</Routes>
		</Router>
	}
}

#[component]
fn Contract(cx: Scope) -> impl IntoView {
	let action = create_server_action::<Contract>(cx);
	view! { cx,
		<ActionForm action=action>
			<label>
				"Email: "
				<input type="email" name="email" required />
			</label>
			<label>
				"Message: "
				<input type="text" name="message" required />
			</label>
			<input type="submit" value="Submit" />
		</ActionForm>
	}
}

#[server(Contract, "/api")]
async fn post(cx: Scope, email: String, message: String) -> Result<String, ServerFnError> {
	println!("{email} | {message}");
	leptos_actix::redirect(cx, "/");
	Ok("Success!".into())
}
