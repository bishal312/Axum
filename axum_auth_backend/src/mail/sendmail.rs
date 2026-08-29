use std::{env, fs};
use lettre::{
    message::{
        header,
        SinglePart,
    },
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport,
    Transport,
};


pub async fn send_email(
    to_email: &str,
    subject: &str,
    templet_path: &str,
    placeholders: &[(String, String)],
) -> Result<(), Box<dyn std::error::Error>> {
    let smtp_username = env::var("SMTP_USERNAME")?;
    let smtp_password = env::var("SMPT_PASSWORD")?;
    let smtp_server = env::var("SMTP_SERVER")?;
    let smtp_port: u16 = env::var("SMTP_PORT")?.parse()?;

    let mut html_templet = fs::read_to_string(templet_path)?;

    for (key, value) in placeholders {
        html_templet = html_templet.replace(key, value)
    }

    let email = Message::builder()
        .from(smtp_username.parse()?)
        .to(to_email.parse()?)
        .subject(subject)
        .header(header::ContentType::TEXT_HTML)
        .singlepart(SinglePart::builder()
            .header(header::ContentType::TEXT_HTML)
            .body(html_templet)
    )?;

    let  creds = Credentials::new(smtp_username, smtp_password);
    let mailer = SmtpTransport::starttls_relay(&smtp_server)?
        .credentials(creds)
        .port(smtp_port)
        .build();

    let result = mailer.send(&email);
    
    match result {
        Ok(_) => println!("Email sent successful!"),
        Err(e) => println!("Failed to send email: {:?}", e),
    }

    Ok(())
}