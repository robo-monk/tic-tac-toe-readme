<script>
    import { onMount } from "svelte";
    import sha256 from 'crypto-js/sha256';


    let username = "";
    // let href = "http://localhost:3000/"
    // let href = "http://localhost:8787/"
    let href = window.location.origin + window.location.pathname;
    let production = false;
    let baseUrl = "http://localhost:8787"
    let setUsername = (u) => {}

    $: setUsername(username)
    $: href = production ? `https://github.com/${username}` : href
    onMount(() => {
        // baseUrl = window.location.origin
        production = !window.location.origin.includes('localhost:3000')
        // production = true
        baseUrl = production ? "https://tic-tac-toe-readme.rrobomonk.workers.dev" : baseUrl;

        console.log('localstaorgae', localStorage)
        username = username || localStorage.getItem('username') 
        setUsername = (username) => localStorage.setItem('username', username)
    });

    let code = ""
    // const baseUrl = globalThis.location?.origin

    const cellTemplate = (params) => {
        return `
            <a href="${baseUrl}/api/click?${new URLSearchParams(params).toString()}">
                ${imgTemplate(params)}
            </a>
        `

    }
    const imgTemplate = (params) => {
        return `<img src="${baseUrl}/api/cell.svg?${new URLSearchParams(params).toString()}" style='width: 80px; height: 80px; border: 1px solid black;'>`
    };

    const repeat = (cb, times, offset = 0, join=" ") => {
        let acc = [];
        for (let i = offset; i < offset + times; i++) {
            acc.push(cb(i));
        }
        return acc.join(join)
        // return new Array(times + 1).join(str);
    }

    const template = ({
        u, r
    }) => `
        ${repeat((i) => cellTemplate({ u, i, r }), 3, 0)}
        <br>
        ${repeat((i) => cellTemplate({ u, i, r}), 3, 3)}
        <br>
        ${repeat((i) => cellTemplate({ u, i, r }), 3, 6)}
    `.trim()
    $: {
        const userHash = sha256(username);
        code = template({
            u: userHash,
            r: href
        });
    }
</script>

<div class="flex-center">
    <input type="text" placeholder="GitHub username" bind:value={username} />
    <br>
    <div class="preview">
        { @html code }
    </div>

    <br>
    <p class="output">
        { code }
    </p>


</div>

<style>
    .flex-center {
        width: 100%;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
    }
    .output {
        font-size: 14px;
        background-color: black;
        padding: 25px 40px;
        border-radius: 8px;
        word-wrap: break-word;
    }

    input[type="text"] {
        padding: 8px;
        background-color: rgba(245, 245, 245, 0.08);
        font-size: 18px;
        color: whitesmoke;
        border: none;
    }
</style>
