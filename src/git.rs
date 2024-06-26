use chrono::{TimeZone, Utc};

byond_fn!(fn rg_git_revparse(rev) {
    let repo = gix::open(".");
    let repo = repo.as_ref().ok()?;
    let object = repo.rev_parse_single(rev).ok()?;
    Some(object.to_string())
});

byond_fn!(fn rg_git_commit_date(rev) {
    let repo = gix::open(".");
    let repo = repo.as_ref().ok()?;
    let rev = repo.rev_parse_single(rev).ok()?;
    let object = rev.object().ok()?;
    let commit = object.try_into_commit().ok()?;
    let commit_time = commit.committer().ok()?.time;
    let datetime = Utc.timestamp_opt(commit_time.seconds, 0).latest()?;
    Some(datetime.format("%F").to_string())
});
