import {spawn} from 'node:child_process';

const ls = spawn('ls', ['-lh', '/usr']);
const command = spawn('docker', ['exec', '--workdir', '/opt/kafka/bin/', '-it', 'broker', 'sh'], {stdio: 'inherit'})

command.on('close', (code) => {
    console.log(`child process exited with code ${code}`);
});