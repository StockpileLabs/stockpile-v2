# Security Policy

1. [Reporting security issues](#reporting)
2. [Incident Response Process](#process)

<a name="reporting"></a>
## Reporting security problems

**DO NOT CREATE A GITHUB ISSUE** to report a security problem.

Instead please email info@stockpile.so, and provide a helpful title, detailed description of the vulnerability and an exploit
proof-of-concept. Speculative submissions without proof-of-concept will be closed
with no further consideration.

If you haven't done so already, please **enable two-factor auth** in your GitHub account.

Expect a response as fast as possible in the advisory, typically within 72 hours.

--

If you do not receive a response in the advisory, forward the original email to
joey@stockpile.so.

<a name="process"></a>
## Incident Response Process

In case an incident is discovered or reported, the following process will be
followed to contain, respond and remediate:

### 1. Accept the new report
In response a newly reported security problem, a member of the
Stockpile team will respond to the report to learn more.  

If the advisory is the result of an audit finding, follow the same process as above but add the auditor's github user(s) and begin the title with "[Audit]".

If the report is out of scope, a member of the Stockpile team will comment as such and then close the report.

### 2. Triage
Within the draft security advisory, discuss and determine the severity of the issue. If necessary, members of the Stockpile team may add other github users to the advisory to assist.
If it is determined that this not a critical issue then the advisory should be closed and if more follow-up is required a normal public github issue should be created.

### 3. Prepare Fixes
For the affected branches, typically all three (mainnet, devnet and localnet), prepare a fix for the issue and push them to the corresponding branch in the private repository associated with the draft security advisory.
There is no CI available in the private repository so you must manually verify fixes.
Code review from the reporter is ideal, as well as from multiple members of the core development team.

### 4. Ship the patch
Once the fix is accepted, a member of the Stockpile team should prepare a commit for each affected branch. The commit title for the patch should only contain an advisory id, and not disclose any further details about the incident.

### 5. Public Disclosure and Release
Once the fix has been deployed to devnet, the patches from the security advisory may be merged into the mainnet source repository. A new official release for each affected branch should be shipped.
