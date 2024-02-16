use std::error::Error;

use reqwest::blocking::Client;
use serde::Serialize;

use crate::models::Employee;

#[derive(Clone, Debug, Serialize)]
struct DiscordMessageBody {
    username: String,
    avatar_url: String,
    content: String,
    embeds: Vec<DiscordMessageEmbed>,
}

#[derive(Clone, Debug, Serialize)]
struct DiscordMessageEmbed {
    title: String,
    description: String,
    thumbnail: DiscordMessageEmbedThumbnail,
    footer: DiscordMessageEmbedFooter,
}

#[derive(Clone, Debug, Serialize)]
struct DiscordMessageEmbedThumbnail {
    url: String,
}

#[derive(Clone, Debug, Serialize)]
struct DiscordMessageEmbedFooter {
    text: String,
}

pub struct DiscordClient<'a> {
    webhook_url: &'a str,
    bot_username: &'a str,
}

impl<'a> DiscordClient<'a> {
    pub fn new(webhook_url: &'a str, bot_username: &'a str) -> Self {
        Self {
            webhook_url,
            bot_username,
        }
    }

    pub fn send(&self, newcomers: &Vec<Employee>, departures: &Vec<Employee>) -> Result<(), Box<dyn Error>> {
        let body = self.build_message(newcomers, departures);
        let serialized_body = serde_json::to_string(&body)?;

        self.post_message(serialized_body)?;

        Ok(())
    }

    fn build_message(
        &self,
        newcomers: &Vec<Employee>,
        departures: &Vec<Employee>,
    ) -> DiscordMessageBody {
        let content = String::from("Launching into New Adventures!");
        let mut embeds: Vec<DiscordMessageEmbed> = Vec::new();
        let format_employee = |employee: &Employee| {
            let name = format!("- {}", employee.name.clone());

            if employee.workplace.is_none()
                && employee.business_unit.is_none()
                && employee.job_title.is_none()
            {
                return name;
            }

            let mut details: Vec<String> = Vec::new();

            if let Some(w) = &employee.workplace {
                details.push(w.clone());
            };

            if let Some(bu) = &employee.business_unit {
                details.push(bu.clone());
            };

            if let Some(jt) = &employee.job_title {
                details.push(jt.clone());
            };

            format!("{} ({})", name, details.join(" - "))
        };

        if newcomers.len() > 0 {
            embeds.push(DiscordMessageEmbed {
                title: String::from("Warm welcome to"),
                description: String::from(
                    newcomers
                        .iter()
                        .map(format_employee)
                        .collect::<Vec<String>>()
                        .join("\n"),
                ),
                thumbnail: DiscordMessageEmbedThumbnail {
                    url: String::from("https://imgur.com/gkxlcqx.jpg"),
                },
                footer: DiscordMessageEmbedFooter {
                    text: String::from("We are thrilled to have you on board."),
                },
            });
        }

        if departures.len() > 0 {
            embeds.push(DiscordMessageEmbed {
                title: String::from("Fond farewell to"),
                description: String::from(
                    departures
                        .iter()
                        .map(format_employee)
                        .collect::<Vec<String>>()
                        .join("\n"),
                ),
                thumbnail: DiscordMessageEmbedThumbnail {
                    url: String::from("https://imgur.com/Ga9bZ7q.jpg"),
                },
                footer: DiscordMessageEmbedFooter {
                    text: String::from("Wishing you the best in your future endeavors."),
                },
            });
        }

        DiscordMessageBody {
            username: String::from(self.bot_username),
            avatar_url: String::from("https://i.imgur.com/o7XM6SF.png"),
            content,
            embeds,
        }
    }

    fn post_message(&self, payload: String) -> Result<(), Box<dyn Error>> {
        let client = Client::builder().build()?;

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());

        let request = client
            .request(
                reqwest::Method::POST,
                self.webhook_url
            )
            .headers(headers)
            .body(payload);

        let response = request.send()?;

        if !response.status().is_success() {
            return Err(Box::from(format!(
                "Discord client request error {}",
                response.status().as_str()
            )));
        }

        Ok(())
    }
}
