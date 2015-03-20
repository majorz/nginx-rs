#include <ngx_config.h>
#include <ngx_core.h>
#include <ngx_http.h>


char *ngx_http_sample_module_command(ngx_conf_t *cf, ngx_command_t *cmd, void *conf);

static ngx_command_t  ngx_http_sample_commands[] = {
   {
      ngx_string("sample"),
      NGX_HTTP_LOC_CONF|NGX_CONF_NOARGS,
      ngx_http_sample_module_command,
      NGX_HTTP_LOC_CONF_OFFSET,
      0,
      NULL
   },

   ngx_null_command
};

ngx_http_module_t  ngx_http_sample_module_ctx = {
   NULL,                                  /* preconfiguration */
   NULL,                                  /* postconfiguration */

   NULL,                                  /* create main configuration */
   NULL,                                  /* init main configuration */

   NULL,                                  /* create server configuration */
   NULL,                                  /* merge server configuration */

   NULL,                                  /* create location configration */
   NULL                                   /* merge location configration */
};

ngx_module_t  ngx_http_sample_module = {
   NGX_MODULE_V1,
   &ngx_http_sample_module_ctx,           /* module context */
   ngx_http_sample_commands,              /* module directives */
   NGX_HTTP_MODULE,                       /* module type */
   NULL,                                  /* init master */
   NULL,                                  /* init module */
   NULL,                                  /* init process */
   NULL,                                  /* init thread */
   NULL,                                  /* exit thread */
   NULL,                                  /* exit process */
   NULL,                                  /* exit master */
   NGX_MODULE_V1_PADDING
};
