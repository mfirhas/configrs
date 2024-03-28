async function getDownloadUrl(
  releaseEndpoint: string,
  requestedVersion: string,
): Promise<string> {
  const releaseInfoUri =
    requestedVersion === "latest"
      ? `${releaseEndpoint}/latest`
      : `${releaseEndpoint}/tags/${requestedVersion}`;

  const releaseInfoRequest = await fetch(releaseInfoUri);
  const releaseInfo = await releaseInfoRequest.json();
  const asset = releaseInfo["assets"].find((asset) => {
    return asset["content_type"] === "application/gzip";
  });

  if (!asset) {
    throw new Error(
      `Couldn't find a release tarball containing binaries for ${requestedVersion}`,
    );
  }

  return asset["browser_download_url"];
}
