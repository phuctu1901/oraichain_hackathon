import _ from "https://deno.land/std@0.120.0/node/module.ts";

const httpPost = async (fileName) => {
    const url = "http://45.76.158.89:5000/process";
    // Build formData object.
    let formData = new FormData();
    formData.append('image', fileName);
    const data = await fetch(url, {
        method: 'POST',
        body: formData
    });
    return data;
}

const main = async (data) => {
    console.log(data)

    let result = await httpPost(data)
    console.log(result);
    // console.log(JSON.stringify(responses))
};

main(...process.argv.slice(2))