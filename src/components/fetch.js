export const useFetch = async (url, method = 'GET', payload = null) => {
  const opts = { method }
  if (payload) {
    opts.headers = {
      'Content-Type': 'application/json',
    }
    opts.body = JSON.stringify(payload)
  }
  const res = await fetch(url, opts)
  if (res.status != 200) {
    return res.statusText
  }
  // if (Error in res) {
  //   connected.value = false
  // } else {
  //   connected.value = true
  // }

  return res.json()
}
