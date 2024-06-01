use maud::{html, Markup, PreEscaped};
use css_minify::optimizations::{Minifier, Level};
use rocket::response::Redirect;
use rocket::{get, routes, launch};
use rocket::fs::FileServer;

macro_rules! relative {
	($path: expr) => (concat!(env!("CARGO_MANIFEST_DIR"), $path))
}

macro_rules! include_static {
	($path: expr) => (include_str!(relative!(concat!("/public", $path))))
}

macro_rules! include_static_unsafe {
	($path: expr) => (PreEscaped(include_static!($path)))
}

macro_rules! include_css {
	($path: expr) => (PreEscaped(Minifier::default().minify(include_static!($path), Level::Three).unwrap()))
}

fn base(content: Markup) -> Markup {
    html! {
        html lang="fr"{
            head {
                meta http-equiv="Content-Type" content="text/html; charset=UTF-8";
                meta charset="utf-8";
                meta name="robots" content="index, follow";
                meta name="theme-color" content="#000000";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                meta http-equiv="keywords" name="keywords" content="Discord, discord, epl, students, école, polytechnique, louvain, ingénieur, cercle, industriel";
                meta http-equiv="description" name="description" content="EPLStudents est un site recensant les projets de la communauté étudiante de l'EPL.";
                meta http-equiv="cache-control" content="no-cache";
                meta http-equiv="pragma" content="no-cache";
                
                link rel="icon" href="/public/favicon.ico";

                style {
					(include_css!("/main.css"))
				}
                title { "EPLStudents" }
            }
            body {
                (content)
            }
        }
    }

}

fn project(title: &'static str, link: &'static str, img_src: &'static str, descr: Markup) -> Markup {
	let alt: &str = &(title.to_owned() + " thumbnail");

	html! {
		.project {
			.labeled-img {
				img alt=(alt) src=(img_src);
				div {
					div {
						h2 { (title) }
					}
				}
			}
			p { (descr) }
			a.learn-more href=(link) {
				p { "Voir" }
			}
		}
	}
}

fn button(handle: &'static str, link: &'static str, icon: PreEscaped<&str>) -> Markup {
	html! {
		a.button href=(link) {
			(icon)
			p { (handle) }
		}
	}
}

const EPLWIKI_IMG_SRC:   		&str = "/public/thumbnails/eplwiki.png";
const DISCORD_IMG_SRC:   		&str = "/public/thumbnails/discord.png";
const DISCORD_SINF_IMG_SRC:   	&str = "/public/thumbnails/discord_sinf.png";

#[get("/")]
fn index() -> Markup {
	base(html! {
		.container {
			header {
				h1 { "EPLStudents" }
			}
            .separation {}
			main {
				p {
					"Ce site recense tous les projets inventés par la communauté étudiante de l'EPL. Vous y trouverez des projets de cours, des projets personnels, ou même des projets à destination de la communauté comme ce site 🙃."
				}
				.projects {
					(project("EPLWiki", "https://wiki.epl-students.be", EPLWIKI_IMG_SRC, html! {
						"Un recueil d'explications, de conseils et d'astuces sur la vie à l'EPL. "
					}))

					(project("Discord EPL", "https://discord.epl-students.be", DISCORD_IMG_SRC, html! {
						"Le Discord des étudiants ingénieurs de l'EPL. N'hésitez pas à nous rejoindre !"
					}))

					(project("Discord SINF", "https://discord.gg/eR3WcnPBxt", DISCORD_SINF_IMG_SRC, html! {
						"Le Discord des étudiants SINF de l'EPL. N'hésitez pas à nous rejoindre !"
					}))
				}
			}

			footer {
                p { "Ce site est open-source et maintenu par la communauté étudiante de l'EPL." }
				.buttons {
					(button("Source code", "https://github.com/EPLStudents/eplstudents.github.io", include_static_unsafe!("/icons/gh.svg")))
				}
			}
		}
	})
}

#[get("/discord")]
fn discord() -> Redirect {
    Redirect::to("https://discord.gg/3ZH2YWhsCa")
}

#[get("/discord_SINF")]
fn discord_sinf() -> Redirect {
    Redirect::to("https://discord.gg/eR3WcnPBxt")
}

#[launch]
fn rocket() -> _ {
	let rocket = rocket::build();

	rocket
		.mount("/", routes![index, discord])
		.mount("/public", FileServer::from(relative!("/public")))
}