import { ref } from 'vue'

export function useFetch(url, data, error, method="GET", payload=null, callback=null) {

  const opts = {method}
  if (payload) {
    opts.headers = {
      'Content-Type': 'application/json',
    }
    console.log("PAYLOAD " + JSON.stringify(payload));
    opts.body = JSON.stringify(payload);

    console.log(opts)
  }

  fetch(url, opts)
  .then((res) => {
    if (res.status != 200) {
      throw(res.statusText);
    }
    return res.json();
  })
  .then((json) => {
      data.value = json;
      if (callback) callback();
  })
  .catch((err) => {
      error.value = err
  })

}