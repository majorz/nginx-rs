#include <ngx_config.h>
#include <ngx_core.h>
#include <ngx_http.h>


ngx_str_t sample_text_from_rust(ngx_http_request_t *r);


static char *ngx_http_sample_module_command(ngx_conf_t *cf, ngx_command_t *cmd, void *conf);


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

//static ngx_str_t ngx_http_sample_text = ngx_string(
//   "ngx_http_sample_handler"
//);

static ngx_int_t
ngx_http_sample_handler(ngx_http_request_t *r)
{
   ngx_int_t     rc;
   ngx_buf_t    *b;
   ngx_chain_t   out;

   ngx_str_t ngx_http_sample_text;

   ngx_http_sample_text = sample_text_from_rust(r);

   r->headers_out.status = NGX_HTTP_OK;
   r->headers_out.content_length_n = ngx_http_sample_text.len;

   rc = ngx_http_send_header(r);

   if (rc == NGX_ERROR || rc > NGX_OK || r->header_only) {
      return rc;
   }

   b = ngx_calloc_buf(r->pool);
   if (b == NULL) {
      return NGX_HTTP_INTERNAL_SERVER_ERROR;
   }

   out.buf = b;
   out.next = NULL;

   b->start = b->pos = ngx_http_sample_text.data;
   b->end = b->last = ngx_http_sample_text.data + ngx_http_sample_text.len;
   b->memory = 1;
   b->last_buf = 1;
   b->last_in_chain = 1;

   return ngx_http_output_filter(r, &out);
}

static char *
ngx_http_sample_module_command(ngx_conf_t *cf, ngx_command_t *cmd, void *conf)
{
   ngx_http_core_loc_conf_t  *clcf;

   clcf = ngx_http_conf_get_module_loc_conf(cf, ngx_http_core_module);
   clcf->handler = ngx_http_sample_handler;

   return NGX_CONF_OK;
}
