import { spawn } from 'node:child_process';

const command = spawn(
    'docker',
    [
        'exec',
        '-it', // interactive mode
        '--workdir', '/opt/kafka/bin/',
        'broker',
        'sh', '-c', './kafka-console-consumer.sh --bootstrap-server localhost:9092 --topic test --from-beginning'
    ],
    { stdio: 'inherit' } // hand over terminal directly
);

command.on('close', (code) => {
    console.log(`child process exited with code ${code}`);
});
