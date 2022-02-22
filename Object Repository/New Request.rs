<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>New Request</name>
   <tag></tag>
   <elementGuidId>d0a0f33c-fc53-4775-8dc8-7c12933f4e85</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>384</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzUxMiJ9.eyJhdmF0YXJfdXJsIjoiaHR0cHM6Ly9saDMuZ29vZ2xldXNlcmNvbnRlbnQuY29tL2EtL0FPaDE0R2gtNWo1Uk82Zm9oMUtJcUE0NDQ4aXFLWlFTUDBHMUxQNDZnTkZvPXM5Ni1jIiwicm9sZXMiOlsiUk9MRV9URVFJRSIsIlJPTEVfU1VQRVJfQURNSU4iLCJST0xFX0FETUlOIiwiUk9MRV9NQU5BR0VSIiwiUk9MRV9TRVJWSUNFX0FETUlOIl0sImlzcyI6ImVtc19hdXRoX3NlcnZpY2UiLCJsYXN0X25hbWUiOiJEemlhZGtvdXNrYXlhIiwiZXhwIjoxNjQ1NDYxNDg5LCJmaXJzdF9uYW1lIjoiWWFuYSIsImVtYWlsIjoieWFuYS5kemlhZGtvdXNrYXlhQHNvZnRlcS5jb20ifQ.Bz7sQ5iy8sPAh84HApycAPtRBb96TvJFqdd7IY-lTHVDe7X2LfasPB6MzHCZqtPDSTkrMCZEpSRugKqt5u-nBsMJ6_23vWzPUDX9lsnyWBm5CcpVgcRNRBRxv1cVSnZlhRotTGuK3dEju6yMXuyUFju1-zXt6oz13xAffPVxiy7vZbfrAnqQpKLr0ZejQIuNrBiQH1GlM0_I3IcuUvktmzmQB9irpv1wdhdddt9r1fZVds-octMctifvxgA4MMaWOTz-vnwwnYJzUYwqSJ-PrjI0YLQnC_lPQA2yCxMNkaenauEpcvgTEbgcryQdUFW8O794htqQz6owskStguypMrtTX9Y04AadPpY0WV3JNS_SOGO7J4eh7W4y0D4pVsKnItZmbgzpKaOxfB_9AntL0YtgRpcHv9I5oNPcwOKwQ-PvmD-ot7kGyL_gXTVrPo4qghk-QWNafysrzJQnc_ymUvs9cOATfv_fH0god4jCTePiuIJ1Ehpu7GF-86uOz0gLD_scK-fjbAa6agkQNMDBwihZztCQ0mvSP5I7f1MCNThYV-ImQpTXfoZm_ovpbNQP6ja57b_yw3wu4ts3vP-yzt-vsEujCKAdOH80wOP6NTsKDXV2A9sq8Ox2ihQDoWyaX9RuM1nhaVt1xeZtUz10JbcYOSjhHBWLGbkv4OzfDy0</value>
   </httpHeaderProperties>
   <katalonVersion>8.2.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://localhost:8080/api/v1/teqies/382</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)


def variables = request.getVariables()
def variable = variables.get('yourVariableName')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
