window.onload = function() {
  //<editor-fold desc="Changeable Configuration Block">

  // get the origin from the current url
  const origin = window.location.origin;

  // the following lines will be replaced by docker/configurator, when it runs in a docker-container
  window.ui = SwaggerUIBundle({
    // url: "https://petstore.swagger.io/v2/swagger.json",
    urls: [
      {
        name: "Auth",
        url: `${origin}/api/openapi-files/auth/openapi.yml`
      },
      {
        name: "Profile",
        url: `${origin}/api/openapi-files/profile/openapi.yml`
      }
    ],
    dom_id: '#swagger-ui',
    deepLinking: true,
    presets: [
      SwaggerUIBundle.presets.apis,
      SwaggerUIStandalonePreset
    ],
    plugins: [
      SwaggerUIBundle.plugins.DownloadUrl
    ],
    layout: "StandaloneLayout"
  });

  //</editor-fold>
};
