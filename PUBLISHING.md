# Local development setup

The repository is already published at **https://github.com/kudzimusar/rese-rese**. The original bundle/script bootstrap instructions are obsolete.

On the Owner's Mac:
```bash
mkdir -p "/Users/shadreckmusarurwa/Project AI"
cd "/Users/shadreckmusarurwa/Project AI"
git clone https://github.com/kudzimusar/rese-rese.git
cd rese-rese
git status
```

For subsequent updates:
```bash
cd "/Users/shadreckmusarurwa/Project AI/rese-rese"
git pull --ff-only
```

Read `README.md`, `AGENTS.md`, `docs/AUTHORITY.md`, the roadmap and the relevant ADRs before coding. Never push sensitive credentials, and do not touch Tengasell or Production without separate Owner authorization.
