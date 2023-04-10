allow_k8s_contexts(['knative'])
k8s_kind(
    'Service',
    api_version='serving.knative.dev/v1',
    image_json_path='{.spec.template.spec.containers[*].image}'
)

local_resource(
  'wasm',
  'just wasm',
  ['src/wasm'],
  trigger_mode=TRIGGER_MODE_MANUAL
)

local_resource(
  'containers',
  'just containers',
  ['src/containers'],
  trigger_mode=TRIGGER_MODE_MANUAL
)

def knative_fn(group, function):
  name=group+"-"+function

  custom_build(
    name,
    'just fn_image %s $EXPECTED_REF' % name,
    ['src/wasm/bin/%s' % name]
  )

  k8s_yaml(helm(
    'deploy/app',
    name=name,
    namespace='default',
    set=[
      "name=%s" % name,
      "image=%s" % name
    ],
  ))

  k8s_resource(
    name,
    resource_deps=['wasm']
  )

def operator(name):
  custom_build(
    name,
    'just container_image %s $EXPECTED_REF' % name,
    ['src/containers/bin/%s' % name]
  )

  k8s_yaml(helm(
    'deploy/operator',
    name=name,
    namespace='default',
    values = [ 'src/containers/%s/values.yaml' % name ],
    set=[ "image=%s" % name ],
  ))

  k8s_resource(
    name,
    resource_deps=['containers']
  )

operator('db-operator')
knative_fn('auth', 'get-login')
knative_fn('draughts', 'get-games')