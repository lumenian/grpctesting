<script>
    import { invoke } from "@tauri-apps/api/tauri";

    let goTime;
    let rustTime;
    let totalTime;
    let jsTime;

    async function handleClick() {
        let data;
        goTime = 0;
        rustTime = 0;
        totalTime = 0;
        jsTime = 0;

        const start = performance.now();
        try {
            data = await invoke("my_custom_command");
        } catch (e) {
            console.log("NEW ERROR");
            console.log(e);
        }
        console.log(data);
        const end = performance.now();

        goTime = data.go_time;
        rustTime = data.rust_time;
        totalTime = (end - start) * 1000;
        console.log(totalTime);
        jsTime = totalTime - rustTime;
    }
    // export let name;
</script>

<main>
    <h1>Simple gRPC Test App</h1>
    <p>
        This app uses Tauri commands to send a gRPC message to a Go "app
        backend". For testing purposes, 200 users is being fetched.
    </p>
    <button on:click={handleClick}> Call Custom Command </button>
    <p>Go Time {goTime || 0}μs</p>
    <p>Rust Time {rustTime || 0}μs</p>
    <p>JS Time {jsTime || 0}μs</p>
    <p>TOTAL TIME: {totalTime || 0}μs</p>
</main>

<style>
    main {
        text-align: center;
        padding: 1em;
        max-width: 240px;
        margin: 0 auto;
    }

    h1 {
        color: #ff3e00;
        text-transform: uppercase;
        font-size: 4em;
        font-weight: 100;
    }

    @media (min-width: 640px) {
        main {
            max-width: none;
        }
    }
</style>
