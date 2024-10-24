(module 
  (span start: 0 end: 1049) 
  name: campaign
  base: https://advertising.amazon.com/api-model
  body: (module_body 
    (span start: 59 end: 1048) 
    (import 
      (span start: 65 end: 117) 
      (module_import 
        (span start: 78 end: 80) 
        name: dc)
      (module_import 
        (span start: 85 end: 88) 
        name: owl)
      (module_import 
        (span start: 93 end: 96) 
        name: rdf)
      (module_import 
        (span start: 101 end: 105) 
        name: skos)
      (module_import 
        (span start: 110 end: 113) 
        name: xsd))
    (annotation 
      (span start: 122 end: 152) 
      name: (identifier_reference
        (qualified_identifier 
          (span start: 123 end: 135) 
          module: 'skos
          member: 'version))
      value: xsd:decimal(2))
    (data_type_def 
      (span start: 192 end: 275) 
      name: (identifier (span start: 201 end: 205) 'Name)
      base: (identifier_reference
        (qualified_identifier 
          (span start: 209 end: 219) 
          module: 'xsd
          member: 'string))
      body: (annotation_only_body 
        (span start: 220 end: 275) 
        (annotation 
          (span start: 227 end: 245) 
          name: (identifier_reference
            (qualified_identifier 
              (span start: 228 end: 241) 
              module: 'xsd
              member: 'minLength))
          value: 5)
        (annotation 
          (span start: 250 end: 269) 
          name: (identifier_reference
            (qualified_identifier 
              (span start: 251 end: 264) 
              module: 'xsd
              member: 'maxLength))
          value: 25)))
    (data_type_def 
      (span start: 279 end: 416) 
      name: (identifier (span start: 288 end: 298) 'CampaignId)
      base: (identifier_reference
        (qualified_identifier 
          (span start: 302 end: 312) 
          module: 'xsd
          member: 'string))
      body: (annotation_only_body 
        (span start: 313 end: 416) 
        (annotation 
          (span start: 320 end: 410) 
          name: (identifier_reference
            (qualified_identifier 
              (span start: 321 end: 335) 
              module: 'skos
              member: 'prefLabel))
          value: ["Campaign Identifier"@en "Identified de campagne"@fr])))
    (enum_def 
      (span start: 420 end: 609) 
      name: (identifier (span start: 425 end: 430) 'State)
      body: (enum_body 
        (span start: 431 end: 609) 
        (annotation 
          (span start: 438 end: 474) 
          name: (identifier_reference
            (qualified_identifier 
              (span start: 439 end: 458) 
              module: 'owl
              member: 'equivalentClass))
          value: sdml:unsigned)
        (value_variant 
          (span start: 479 end: 518) 
          name: (identifier (span start: 479 end: 486) 'Running)
          value: (annotation_only_body 
            (span start: 487 end: 518) 
            (annotation 
              (span start: 496 end: 510) 
              name: (identifier_reference
                (qualified_identifier 
                  (span start: 497 end: 506) 
                  module: 'rdf
                  member: 'value))
              value: 1)))
        (value_variant 
          (span start: 523 end: 561) 
          name: (identifier (span start: 523 end: 529) 'Paused)
          value: (annotation_only_body 
            (span start: 530 end: 561) 
            (annotation 
              (span start: 539 end: 553) 
              name: (identifier_reference
                (qualified_identifier 
                  (span start: 540 end: 549) 
                  module: 'rdf
                  member: 'value))
              value: 2)))
        (value_variant 
          (span start: 566 end: 603) 
          name: (identifier (span start: 566 end: 571) 'error)
          value: (annotation_only_body 
            (span start: 572 end: 603) 
            (annotation 
              (span start: 581 end: 595) 
              name: (identifier_reference
                (qualified_identifier 
                  (span start: 582 end: 591) 
                  module: 'rdf
                  member: 'value))
              value: 3)))))
    (structure_def 
      (span start: 613 end: 692) 
      name: (identifier (span start: 623 end: 626) 'Tag)
      body: (structure_body 
        (span start: 627 end: 692) 
        (member 
          (span start: 634 end: 652) 
          name: (identifier (span start: 634 end: 637) 'key)
          target: (identifier_reference
            (qualified_identifier 
              (span start: 641 end: 652) 
              module: 'xsd
              member: 'NMTOKEN))
          cardinality: (cardinality_expression min: 1 max: 1))
        (member 
          (span start: 657 end: 686) 
          name: (identifier (span start: 657 end: 662) 'value)
          target: (identifier_reference
            (qualified_identifier 
              (span start: 672 end: 686) 
              module: 'rdf
              member: 'langString)))))
    (entity_def 
      (span start: 696 end: 705) 
      name: (identifier (span start: 703 end: 705) 'Ad))
    (entity_def 
      (span start: 709 end: 723) 
      name: (identifier (span start: 716 end: 723) 'AdGroup))
    (entity_def 
      (span start: 751 end: 892) 
      name: (identifier (span start: 758 end: 766) 'Campaign)
      name: (entity_body 
        (span start: 767 end: 892) 
        identity: (entity_identity 
          (span start: 774 end: 807) 
          target: (identifier_reference
            (identifier (span start: 797 end: 807) 'CampaignId)))
        (member 
          (span start: 813 end: 828) 
          name: (identifier (span start: 813 end: 817) 'name)
          target: (unknown_type)
          cardinality: (cardinality_expression min: 1 max: 1))
        (member 
          (span start: 834 end: 850) 
          name: (identifier (span start: 834 end: 837) 'tag)
          target: (identifier_reference
            (identifier (span start: 847 end: 850) 'Tag)))
        (member 
          (span start: 856 end: 886) 
          name: (identifier (span start: 856 end: 862) 'target)
          target: (identifier_reference
            (identifier (span start: 872 end: 886) 'TargetCriteria)))))
    (entity_def 
      (span start: 896 end: 917) 
      name: (identifier (span start: 903 end: 917) 'AudienceTarget))
    (entity_def 
      (span start: 921 end: 944) 
      name: (identifier (span start: 928 end: 944) 'GeographicTarget))
    (union_def 
      (span start: 948 end: 1043) 
      name: (identifier (span start: 954 end: 968) 'TargetCriteria)
      body: (union_body 
        (span start: 969 end: 1043) 
        (type_variant 
          (span start: 976 end: 1002) 
          name: (identifier_reference
            (identifier (span start: 976 end: 990) 'AudienceTarget))
          rename: (identifier (span start: 994 end: 1002) 'Audience))
        (type_variant 
          (span start: 1007 end: 1037) 
          name: (identifier_reference
            (identifier (span start: 1007 end: 1023) 'GeographicTarget))
          rename: (identifier (span start: 1027 end: 1037) 'Geographic))))))
