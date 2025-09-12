// scaffold.js
import {argv} from "node:process";
import {appendFile, mkdirSync, writeFileSync} from "node:fs";

const fileList = ["mod", "service", "middleware", "repository", "router", "handlers"];
const extension = ".rs";

const moduleName = argv[2];

if (!moduleName) {
    console.error("❌ Please provide a module name, e.g.: node scaffold.js my_module");
    process.exit(1);
}

mkdirSync(moduleName, {recursive: false});

fileList.forEach((file) => {
    if (file === "mod") {
        const content = `
       pub mod service;
       pub mod middleware;
       pub mod repository;
       pub mod adapter;
       pub mod router;
       pub mod handler;
       `
        writeFileSync(`${moduleName}/${file}${extension}`, content, "utf8");
    }
    writeFileSync(`${moduleName}/${file}${extension}`, "", "utf8");
});

console.log(`✅ Module '${moduleName}' created with files: ${fileList.join(", ")}`);


// add it to lib.rs
appendFile("lib.rs", `pub mod ${moduleName}; `, "utf8", (err) => {
    if (err) throw err
})
