import { ref } from 'vue'

export function useFetch(url) {
  let data = null
  const error = ref(null)

  fetch(url, {
    method: "GET",
    headers: {
        "Accept": "application/json"
    }
  })
    .then((res) => res.json())
    .then((json) => {
        data = json
    })
    .catch((err) => {
        error.value = err
    })

  return { data, error }
}