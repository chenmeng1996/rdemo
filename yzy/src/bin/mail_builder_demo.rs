use mail_builder::{MessageBuilder, headers::address::Address, mime::MimePart};

fn main() {
    // Build a nested multipart message
    let email = MessageBuilder::new()
        .from(Address::new_address("John Doe".into(), "john@doe.com"))
        .to(Address::new_address("Jane Doe".into(), "jane@doe.com"))
        .subject("Nested multipart message")

        // Define the nested MIME body structure
        .body(MimePart::new_multipart(
            "multipart/mixed",
            vec![
                MimePart::new_text("Part A contents go here...").inline(),
                MimePart::new_multipart(
                    "multipart/mixed",
                    vec![
                        MimePart::new_multipart(
                            "multipart/alternative",
                            vec![
                                MimePart::new_multipart(
                                    "multipart/mixed",
                                    vec![
                                        MimePart::new_text("Part B contents go here...").inline(),
                                        MimePart::new_binary(
                                            "image/jpeg",
                                            "Part C contents go here...".as_bytes(),
                                        )
                                        .inline(),
                                        MimePart::new_text("Part D contents go here...").inline(),
                                    ],
                                ),
                                MimePart::new_multipart(
                                    "multipart/related",
                                    vec![
                                        MimePart::new_html("Part E contents go here...").inline(),
                                        MimePart::new_binary(
                                            "image/jpeg",
                                            "Part F contents go here...".as_bytes(),
                                        ),
                                    ],
                                ),
                            ],
                        ),
                        MimePart::new_binary("image/jpeg", "Part G contents go here...".as_bytes())
                            .attachment("image_G.jpg"),
                        MimePart::new_binary(
                            "application/x-excel",
                            "Part H contents go here...".as_bytes(),
                        ),
                        MimePart::new_binary(
                            "x-message/rfc822",
                            "Part J contents go here...".as_bytes(),
                        ),
                    ],
                ),
                MimePart::new_text("Part K contents go here...").inline(),
            ],
        ))
        .write_to_string()
        // Write the message to a file
        // .write_to(File::create("nested-message.eml").unwrap())
        .unwrap();

    println!("{}", email)
}
