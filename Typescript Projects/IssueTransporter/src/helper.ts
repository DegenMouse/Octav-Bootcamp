import { exec } from 'child_process';
import { promisify } from 'util';

const execAsync = promisify(exec);

interface Label {
    name: string;
    color: string;
    description?: string;
}

interface Issue {
    number: number;
    body: string;
    state: string;
    labels: Label[];
}

const ghCommands = {
    listIssues: (repo: string) => 
        `gh issue list --repo "${repo}" --json number,body,state,labels --state all`,
    listLabels: (repo: string) => 
        `gh label list --repo "${repo}" --json name`,
    createLabel: (repo: string, label: Label) => 
        `gh label create "${label.name}" --color "${label.color}" --repo "${repo}" ${
            label.description ? `--description "${label.description}"` : ''
        }`,
    transferIssue: (issueNumber: number, sourceRepo: string, targetRepo: string) => 
        `gh issue transfer "${issueNumber}" "${targetRepo}" --repo "${sourceRepo}"`,
    editIssueLabels: (issueNumber: number, repo: string, labels: string) => 
        `gh issue edit ${issueNumber} --repo "${repo}" --add-label "${labels}"`
} as const;

async function execGhCommand(cmd: string): Promise<string> {
    try {
        const { stdout, stderr } = await execAsync(cmd);
        if (stderr) console.error('Command error:', stderr);
        return stdout;
    } catch (error: any) {
        throw new Error(`GitHub CLI error: ${error.message}`);
    }
}

async function ensureLabelsExist(labels: Label[], targetRepo: string): Promise<void> {
    const stdout = await execGhCommand(ghCommands.listLabels(targetRepo));
    const existingLabels = new Set(JSON.parse(stdout).map((l: { name: string }) => l.name));
    
    for (const label of labels) {
        if (!existingLabels.has(label.name)) {
            try {
                await execGhCommand(ghCommands.createLabel(targetRepo, label));
                console.log(`  - Created label: ${label.name}`);
            } catch (error) {
                console.error(`Failed to create label "${label.name}"`);
            }
        }
    }
}

async function addLabelsToIssue(newNumber: number, labels: Label[], repo: string) {
    if (labels.length === 0) return;
    
    await new Promise(resolve => setTimeout(resolve, 2000));
    const labelNames = labels.map(l => l.name).join(',');
    await execGhCommand(ghCommands.editIssueLabels(newNumber, repo, labelNames));
    console.log(`  - Added labels to issue #${newNumber}: ${labelNames}`);
}

export async function getIssueNumbers(repo: string): Promise<Issue[]> {
    const stdout = await execGhCommand(ghCommands.listIssues(repo));
    const issues = JSON.parse(stdout);
    console.log(`Found ${issues.length} total issues (both open and closed)`);
    return issues;
}

function extractLinkedIssues(body: string): number[] {
    return [...body.matchAll(/#(\d+)/g)].map(match => parseInt(match[1]));
}

export async function transferIssue(sourceRepo: string, issues: Issue[], targetRepo: string) {
    const results = { succesfull: [] as number[], unsuccesfull: [] as number[] };
    
    // Prepare labels
    const allLabels = new Set(issues.flatMap(issue => issue.labels));
    await ensureLabelsExist([...allLabels], targetRepo);
    
    // Map parent-child relationships
    const issueMap = new Map<number, number[]>(
        issues
            .map(issue => [issue.number, extractLinkedIssues(issue.body)] as [number, number[]])
            .filter(([_, linked]) => linked.length > 0)
    );

    // Sort issues (children first, then parents)
    const sortedIssues = [...issues].sort((a, b) => 
        Number(issueMap.has(a.number)) - Number(issueMap.has(b.number))
    );

    for (const issue of sortedIssues) {
        try {
            const stdout = await execGhCommand(
                ghCommands.transferIssue(issue.number, sourceRepo, targetRepo)
            );

            const newNumber = stdout.match(/Issue #(\d+) transferred/)?.[1];
            console.log(`Issue #${issue.number} transferred to ${targetRepo}${newNumber ? ` as #${newNumber}` : ''}`);
            
            if (issue.labels.length > 0 && newNumber) {
                await addLabelsToIssue(parseInt(newNumber), issue.labels, targetRepo);
            }

            if (issueMap.has(issue.number)) {
                console.log(`  - Transferred with sub-issues: #${issueMap.get(issue.number)?.join(', #')}`);
            }

            results.succesfull.push(issue.number);
        } catch (error) {
            console.error(`Failed to transfer issue #${issue.number}`);
            results.unsuccesfull.push(issue.number);
        }
    }

    return results;
}