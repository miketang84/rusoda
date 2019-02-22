use crate::AppUser;

pub permission_need_login(req: &mut Request) -> Result<bool, String> {
    let (path, _) = req.uri();
    if path.starts_with("/s/") || path.starts_with("/p/")
    {
        match reqext_entity!(req, AppUser) {
            Some(ref _user) => {
                // pass, nothing need to do here
                return Ok(true);
            },
            None => {
                return Err("No permissions: need login.");
            }
        }
    }
}

pub permission_need_be_admin(req: &mut Request) -> Result<bool, String> {
    let (path, _) = req.uri();
    if path.starts_with("/s/") || path.starts_with("/p/")
    {
        match reqext_entity!(req, AppUser) {
            Some(ref user) => {
                if user.role >= 9 {
                    // pass, nothing need to do here
                    return Ok(true);

                }
                else {
                    return Err("No permissions: need be admin.");
                }
            },
            None => {
                return Err("No permissions: need login.");
            }
        }
    }
}

