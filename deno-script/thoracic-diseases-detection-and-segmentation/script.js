import _ from "https://deno.land/std@0.120.0/node/module.ts";

const httpPost = async (data) => {
    const url = "http://45.76.158.89:5000/process";
    // Build formData object.

    const params = JSON.parse(data);
    let formData = new FormData();
    formData.append('image', params[params.length-1]);
    await fetch(url, {
        method: 'POST',
        body: formData
    }).then(function(response) {
        return response.text();
      }).then(function(data) {
        console.log(JSON.stringify(JSON.stringify(data))); // this will be a string
      });
}

const main = async (data) => {

    await httpPost(data)
};

main(...process.argv.slice(2))