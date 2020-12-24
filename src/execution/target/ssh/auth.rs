use crate::ui::prompt_ssh_password;
use url::Url;
use ssh2::Session;
use log::info;

use crate::error::{Error, BoxedResult};

pub fn auth(session: &mut Session, url: &Url) -> Result<(), Error> {
    let username = match url.username() {
        "" => return Err(
            Error::new(format!("Target `{:?}` must specify a username", url))
        ),
        v => v
    };

    fn auth_via_agent(session: &mut Session, username: &str) -> BoxedResult<()> {
        let mut agent = session.agent().map_err(|e| Error::new(e.to_string()))?;
        agent.connect()?;
        agent.list_identities()?;
        let identities = agent.identities().map_err(|e| Box::new(e))?;

        for identity in identities {
            match agent.userauth(username, &identity) {
                Err(_) => (),
                Ok(_) => return Ok(())
            }
        }

        return Err(Box::new(Error::new(format!("No identiy in agent could authenticate"))))
    }

    fn auth_via_password(
        session: &mut Session,
        url: &Url,
        username: &str
    ) -> BoxedResult<()> {
        let host = match url.host() {
            None => return Err(Box::new(Error::new(format!("URL {} must specify a host!", url)))),
            Some(x) => x.to_string()
        };

        let password = prompt_ssh_password(username, &host)?;
        match session.userauth_password(username, &password) {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e))
        }
    }

    match auth_via_agent(session, username) {
        Ok(_) => return Ok(()),
        Err(e) => info!("SSH agent auth to {:?} failed: {}", url, e)
    }

    match auth_via_password(session, url, username) {
        Ok(_) => return Ok(()),
        Err(e) => info!("SSH password auth to {:?} failed: {}", url, e)
    }

    Err(Error::new(format!("SSH Agent and Password auth to {:?} failed", url)))
}
