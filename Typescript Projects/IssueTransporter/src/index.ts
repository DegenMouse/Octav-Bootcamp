import { exec } from 'child_process';
import { promisify } from 'util';

const execAsync = promisify(exec);

/**
 * Get all issue numbers from a repository
 * 
 * @param repo - The repository in the format "OWNER/REPO"
 * @returns Promise<number[]> - Array of issue numbers
 */
async function getIssueNumbers(repo: string): Promise<number[]> {
    try {
        const cmd = `gh issue list --repo "${repo}" --json number`;
        const { stdout } = await execAsync(cmd);
        const issues = JSON.parse(stdout);
        return issues.map((issue: { number: number }) => issue.number);
    } catch (error: any) {
        console.error('Error fetching issue numbers:', error.message);
        return [];
    }
}

/**
 * Transfer an issue using the GitHub CLI.
 * 
 * @param sourceRepo - The source repository in the format "OWNER/REPO"
 * @param issues - The issue number to transfer
 * @param targetRepo - The target repository in the format "OWNER/REPO"
 */
async function transferIssue(sourceRepo: string, issues: number[], targetRepo: string): Promise<void> {
  
    for (const issue of issues) {
        const cmd = `gh issue transfer "${issue}" "${targetRepo}" --repo "${sourceRepo}"`;
        console.log(`Executing command: ${cmd}`);
        try {
            const { stdout, stderr } = await execAsync(cmd);
            if (stdout) {
                console.log('Command output:', stdout);
            }
            if (stderr) {
                console.error('Command error output:', stderr);
            }
            console.log(`Issue #${issue} from ${sourceRepo} successfully transferred to ${targetRepo}.\n`);
        } catch (error: any) {
            console.error(`Error transferring issue #${issue}:`, error.message);
            continue;
        }
    }
    
}


async function main() {
    const sourceRepo = 'SnavMaster09/TestRepo1';
    const targetRepo = 'SnavMaster09/TestRepo2';
    
    const issueNumbers = await getIssueNumbers(sourceRepo);
    console.log('Found issues:', issueNumbers);

    if (issueNumbers.length === 0) {
        console.log('No issues found to transfer');
        return;
    }

    await transferIssue(sourceRepo, issueNumbers, targetRepo);
}

main().catch(err => console.error(err));
