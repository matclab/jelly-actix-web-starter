## Configuring Email Support
Jelly provides simple setup for transactional email. It's configured to work with [Postmark](https://postmarkapp.com), but it also provide SMTP. 

The mail templates are rendered with the help of
[Tera](https://tera.netlify.app/docs/). Note that both `.html` and `.txt`
templates are required in order to send mail compatible with text clients.

## Setting Up Postmark
- Sign up on [Postmark](https://postmarkapp.com). Do your standard domain configuration pieces as need be, and store your API key in your `.env` file.

The templates in here are verified to work with most common email clients. You can add a logo URL in `layout.html`, and configure anything else as necessary.

## Setting Up SMTP
Configure the appropriate environment variables in the `.env` file. 

Enjoy!
