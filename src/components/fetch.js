import { ref } from 'vue'

export function useFetch(url, data, error, callback) {

  fetch(url)
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