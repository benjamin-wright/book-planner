allow_k8s_contexts(['knative'])
k8s_kind(
    'Service',
    api_version='serving.knative.dev/v1',
    image_json_path='{.spec.template.spec.containers[*].image}'
)

def knative_fn(name):
  custom_build(
    name,
    'just image %s $EXPECTED_REF' % name,
    ['.']
  )

  k8s_yaml(helm(
    'deploy/app',
    name=name,
    namespace='default',
    set=[
      "name=%s" % name,
      "image=%s" % name
    ]
  ))

knative_fn('auth-get-login')
knative_fn('draughts-get-games')