/** @type {import('tailwindcss').Config} */

export const content = {
    files: ["*.html", "./src/**/*.rs"],
    extract: {
        rs: (content) => {
            const result = (content.match(/"(?:[^"\\]|\\.)*"|class:(.*)=/g) ?? [])
                .flatMap(candidate => candidate
                    .replaceAll("\"", "")
                    .replace("class:", "")
                    .split(" ")
                    .map(classCandidate => classCandidate.replace(/=$/g, ""))
                )
                .filter(candidate => candidate != "");
            return result;
        }
    }
};

