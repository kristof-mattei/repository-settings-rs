// PUSH

use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct PushEvent {
    pub(crate) r#ref: String,
    pub(crate) repository: Repository,
    pub(crate) commits: Vec<Commit>,
}

#[derive(Deserialize)]
pub(crate) struct Repository {
    pub(crate) default_branch: String,
}

#[derive(Deserialize)]
pub(crate) struct Commit {
    pub(crate) added: Vec<String>,
    pub(crate) modified: Vec<String>,
}

// {
//   "after": "3129049b624187a8b43e51df429ed5d368f7b366",
//   "base_ref": null,
//   "before": "eef9c9ac271fe9848266951260fb3f7b6c04eb14",
//   "commits": [
//     {
//       "added": [],
//       "author": {
//         "email": "864376+Kristof-Mattei@users.noreply.github.com",
//         "name": "Kristof Mattei",
//         "username": "kristof-mattei"
//       },
//       "committer": {
//         "email": "864376+Kristof-Mattei@users.noreply.github.com",
//         "name": "Kristof Mattei",
//         "username": "kristof-mattei"
//       },
//       "distinct": true,
//       "id": "3129049b624187a8b43e51df429ed5d368f7b366",
//       "message": "change",
//       "modified": [],
//       "removed": [".github/settings.yml"],
//       "timestamp": "2024-07-07T12:53:23-07:00",
//       "tree_id": "ec904f7e5ab69389a824cfd648f4abcade7daab3",
//       "url": "https://github.com/kristof-mattei/repository-settings-test/commit/3129049b624187a8b43e51df429ed5d368f7b366"
//     }
//   ],
//   "compare": "https://github.com/kristof-mattei/repository-settings-test/compare/eef9c9ac271f...3129049b6241",
//   "created": false,
//   "deleted": false,
//   "forced": false,
//   "head_commit": {
//     "added": [],
//     "author": {
//       "email": "864376+Kristof-Mattei@users.noreply.github.com",
//       "name": "Kristof Mattei",
//       "username": "kristof-mattei"
//     },
//     "committer": {
//       "email": "864376+Kristof-Mattei@users.noreply.github.com",
//       "name": "Kristof Mattei",
//       "username": "kristof-mattei"
//     },
//     "distinct": true,
//     "id": "3129049b624187a8b43e51df429ed5d368f7b366",
//     "message": "change",
//     "modified": [],
//     "removed": [".github/settings.yml"],
//     "timestamp": "2024-07-07T12:53:23-07:00",
//     "tree_id": "ec904f7e5ab69389a824cfd648f4abcade7daab3",
//     "url": "https://github.com/kristof-mattei/repository-settings-test/commit/3129049b624187a8b43e51df429ed5d368f7b366"
//   },
//   "installation": {
//     "id": 52581431,
//     "node_id": "MDIzOkludGVncmF0aW9uSW5zdGFsbGF0aW9uNTI1ODE0MzE="
//   },
//   "pusher": {
//     "email": "864376+kristof-mattei@users.noreply.github.com",
//     "name": "kristof-mattei"
//   },
//   "ref": "refs/heads/main",
//   "repository": {
//     "allow_forking": true,
//     "archive_url": "https://api.github.com/repos/kristof-mattei/repository-settings-test/{archive_format}{/ref}",
//     "archived": false,
//     "assignees_url": "https://api.github.com/repos/kristof-mattei/repository-settings-test/assignees{/user}",
//     "blobs_url": "https://api.github.com/repos/kristof-mattei/repository-settings-test/git/blobs{/sha}",
//     "branches_url": "https://api.github.com/repos/kristof-mattei/repository-settings-test/branches{/branch}",
//     "clone_url": "https://github.com/kristof-mattei/repository-settings-test.git",
//     "collaborators_url": "https://api.github.com/repos/kristof-mattei/repository-settings-test/collaborators{/collaborator}",
//     "comments_url": "https://api.github.com/repos/kristof-mattei/repository-settings-test/comments{/number}",
//     "commits_url": "https://api.github.com/repos/kristof-mattei/repository-settings-test/commits{/sha}",
//     "compare_url": "https://api.github.com/repos/kristof-mattei/repository-settings-test/compare/{base}...{head}",
//     "contents_url": "https://api.github.com/repos/kristof-mattei/repository-settings-test/contents/{+path}",
//     "contributors_url": "https://api.github.com/repos/kristof-mattei/repository-settings-test/contributors",
//     "created_at": 1720375005,
//     "default_branch": "main",
//     "deployments_url": "https://api.github.com/repos/kristof-mattei/repository-settings-test/deployments",
//     "description": null,
//     "disabled": false,
//     "downloads_url": "https://api.github.com/repos/kristof-mattei/repository-settings-test/downloads",
//     "events_url": "https://api.github.com/repos/kristof-mattei/repository-settings-test/events",
//     "fork": false,
//     "forks": 0,
//     "forks_count": 0,
//     "forks_url": "https://api.github.com/repos/kristof-mattei/repository-settings-test/forks",
//     "full_name": "kristof-mattei/repository-settings-test",
//     "git_commits_url": "https://api.github.com/repos/kristof-mattei/repository-settings-test/git/commits{/sha}",
//     "git_refs_url": "https://api.github.com/repos/kristof-mattei/repository-settings-test/git/refs{/sha}",
//     "git_tags_url": "https://api.github.com/repos/kristof-mattei/repository-settings-test/git/tags{/sha}",
//     "git_url": "git://github.com/kristof-mattei/repository-settings-test.git",
//     "has_discussions": false,
//     "has_downloads": true,
//     "has_issues": true,
//     "has_pages": false,
//     "has_projects": true,
//     "has_wiki": true,
//     "homepage": null,
//     "hooks_url": "https://api.github.com/repos/kristof-mattei/repository-settings-test/hooks",
//     "html_url": "https://github.com/kristof-mattei/repository-settings-test",
//     "id": 825417252,
//     "is_template": false,
//     "issue_comment_url": "https://api.github.com/repos/kristof-mattei/repository-settings-test/issues/comments{/number}",
//     "issue_events_url": "https://api.github.com/repos/kristof-mattei/repository-settings-test/issues/events{/number}",
//     "issues_url": "https://api.github.com/repos/kristof-mattei/repository-settings-test/issues{/number}",
//     "keys_url": "https://api.github.com/repos/kristof-mattei/repository-settings-test/keys{/key_id}",
//     "labels_url": "https://api.github.com/repos/kristof-mattei/repository-settings-test/labels{/name}",
//     "language": null,
//     "languages_url": "https://api.github.com/repos/kristof-mattei/repository-settings-test/languages",
//     "license": null,
//     "master_branch": "main",
//     "merges_url": "https://api.github.com/repos/kristof-mattei/repository-settings-test/merges",
//     "milestones_url": "https://api.github.com/repos/kristof-mattei/repository-settings-test/milestones{/number}",
//     "mirror_url": null,
//     "name": "repository-settings-test",
//     "node_id": "R_kgDOMTLeJA",
//     "notifications_url": "https://api.github.com/repos/kristof-mattei/repository-settings-test/notifications{?since,all,participating}",
//     "open_issues": 0,
//     "open_issues_count": 0,
//     "owner": {
//       "avatar_url": "https://avatars.githubusercontent.com/u/864376?v=4",
//       "email": "864376+kristof-mattei@users.noreply.github.com",
//       "events_url": "https://api.github.com/users/kristof-mattei/events{/privacy}",
//       "followers_url": "https://api.github.com/users/kristof-mattei/followers",
//       "following_url": "https://api.github.com/users/kristof-mattei/following{/other_user}",
//       "gists_url": "https://api.github.com/users/kristof-mattei/gists{/gist_id}",
//       "gravatar_id": "",
//       "html_url": "https://github.com/kristof-mattei",
//       "id": 864376,
//       "login": "kristof-mattei",
//       "name": "kristof-mattei",
//       "node_id": "MDQ6VXNlcjg2NDM3Ng==",
//       "organizations_url": "https://api.github.com/users/kristof-mattei/orgs",
//       "received_events_url": "https://api.github.com/users/kristof-mattei/received_events",
//       "repos_url": "https://api.github.com/users/kristof-mattei/repos",
//       "site_admin": false,
//       "starred_url": "https://api.github.com/users/kristof-mattei/starred{/owner}{/repo}",
//       "subscriptions_url": "https://api.github.com/users/kristof-mattei/subscriptions",
//       "type": "User",
//       "url": "https://api.github.com/users/kristof-mattei"
//     },
//     "private": false,
//     "pulls_url": "https://api.github.com/repos/kristof-mattei/repository-settings-test/pulls{/number}",
//     "pushed_at": 1720382005,
//     "releases_url": "https://api.github.com/repos/kristof-mattei/repository-settings-test/releases{/id}",
//     "size": 0,
//     "ssh_url": "git@github.com:kristof-mattei/repository-settings-test.git",
//     "stargazers": 0,
//     "stargazers_count": 0,
//     "stargazers_url": "https://api.github.com/repos/kristof-mattei/repository-settings-test/stargazers",
//     "statuses_url": "https://api.github.com/repos/kristof-mattei/repository-settings-test/statuses/{sha}",
//     "subscribers_url": "https://api.github.com/repos/kristof-mattei/repository-settings-test/subscribers",
//     "subscription_url": "https://api.github.com/repos/kristof-mattei/repository-settings-test/subscription",
//     "svn_url": "https://github.com/kristof-mattei/repository-settings-test",
//     "tags_url": "https://api.github.com/repos/kristof-mattei/repository-settings-test/tags",
//     "teams_url": "https://api.github.com/repos/kristof-mattei/repository-settings-test/teams",
//     "topics": [],
//     "trees_url": "https://api.github.com/repos/kristof-mattei/repository-settings-test/git/trees{/sha}",
//     "updated_at": "2024-07-07T19:51:08Z",
//     "url": "https://github.com/kristof-mattei/repository-settings-test",
//     "visibility": "public",
//     "watchers": 0,
//     "watchers_count": 0,
//     "web_commit_signoff_required": false
//   },
//   "sender": {
//     "avatar_url": "https://avatars.githubusercontent.com/u/864376?v=4",
//     "events_url": "https://api.github.com/users/kristof-mattei/events{/privacy}",
//     "followers_url": "https://api.github.com/users/kristof-mattei/followers",
//     "following_url": "https://api.github.com/users/kristof-mattei/following{/other_user}",
//     "gists_url": "https://api.github.com/users/kristof-mattei/gists{/gist_id}",
//     "gravatar_id": "",
//     "html_url": "https://github.com/kristof-mattei",
//     "id": 864376,
//     "login": "kristof-mattei",
//     "node_id": "MDQ6VXNlcjg2NDM3Ng==",
//     "organizations_url": "https://api.github.com/users/kristof-mattei/orgs",
//     "received_events_url": "https://api.github.com/users/kristof-mattei/received_events",
//     "repos_url": "https://api.github.com/users/kristof-mattei/repos",
//     "site_admin": false,
//     "starred_url": "https://api.github.com/users/kristof-mattei/starred{/owner}{/repo}",
//     "subscriptions_url": "https://api.github.com/users/kristof-mattei/subscriptions",
//     "type": "User",
//     "url": "https://api.github.com/users/kristof-mattei"
//   }
// }
