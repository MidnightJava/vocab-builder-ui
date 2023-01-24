
export function useFetch(url, method="GET", payload=null) {

  const opts = {method}
  if (payload) {
    opts.headers = {
      'Content-Type': 'application/json',
    }
    opts.body = JSON.stringify(payload);
  }

  return new Promise((resolve, reject) => {
    fetch(url, opts)
    .then((res) => {
      if (res.status != 200) {
        reject(res.statusText);
      }
      resolve(res.json());
    })
    .catch((err) => {
        reject(err);
    })

  })

  
}