use robotstxt::DefaultMatcher;

pub fn allowed(url: &str, agent: &str) -> bool {
    let mut matcher = DefaultMatcher::default();  // <- musi być mut
    matcher.one_agent_allowed_by_robots("", agent, url)
}
