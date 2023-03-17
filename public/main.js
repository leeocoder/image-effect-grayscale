async function init() {
  let rustApp = null;
  try {
    rustApp = await import('../pkg');
  } catch (error) {
    console.log(error);
    return;
  }
  const input = document.querySelector('#upload');
  const fileReader = new FileReader();

  fileReader.onloadend = () => {
    const base64 = fileReader.result.replace(
      /^data:image\/(png|jpeg|jpg);base64,/,
      ''
    );
    let img_data_url = rustApp.grayscale(base64);
    document.getElementById('new-img').setAttribute('src', img_data_url);
  };
  input.addEventListener('change', () => {
    fileReader.readAsDataURL(input.files[0]);
  });
}

init();
