async function printForever() {
    while (true) {
        console.log((new Date()).toISOString());

        await new Promise(resolve => setTimeout(resolve, 1000));
    }
}

(async () => {
    await printForever();
})();
