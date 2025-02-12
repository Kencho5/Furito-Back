use aws_sdk_sesv2::types::{Body, Content, Destination, EmailContent, Message};
use aws_sdk_sesv2::Error;

pub async fn send_email(
    client: aws_sdk_sesv2::Client,
    recipient: &String,
    subject: String,
    code: Option<String>,
) -> Result<(), Error> {
    let mut html: String = include_str!("code.html").to_string();
    if let Some(code) = code {
        html = html.replace("{{verification_code}}", &code);
    }
    println!("{:?}", html);

    let dest: Destination = Destination::builder().to_addresses(recipient).build();
    let subject_content = Content::builder()
        .data(subject)
        .charset("UTF-8")
        .build()
        .expect("building Content");
    let body_content = Content::builder()
        .data(html)
        .charset("UTF-8")
        .build()
        .expect("building Content");
    let body = Body::builder().html(body_content).build();

    let msg = Message::builder()
        .subject(subject_content)
        .body(body)
        .build();

    let email_content = EmailContent::builder().simple(msg).build();

    let send = client
        .send_email()
        .from_email_address("no-reply@furito.com")
        .destination(dest)
        .content(email_content)
        .send()
        .await?;

    println!("{:?}", send);

    Ok(())
}
