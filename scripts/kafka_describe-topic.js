import { spawn } from 'node:child_process';

const command = spawn(
    'docker',
    [
        'exec',
        '--workdir', '/opt/kafka/bin/',
        'broker',
        'sh', '-c', './kafka-topics.sh  --bootstrap-server localhost:9092 --describe --topic test'
    ],
    { stdio: 'inherit' } // pipe directly to terminal
);

command.on('close', (code) => {
    console.log(`child process exited with code ${code}`);
});



command.on('data', (data) => {
    console.log(`child process exited with code ${data}`);
});
