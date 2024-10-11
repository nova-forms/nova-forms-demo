package com.ajila.detectors;

import com.ajila.ElementType;

import org.apache.pdfbox.pdmodel.PDDocument;
import org.apache.pdfbox.text.PDFTextStripper;

import java.io.IOException;
import java.util.Map;
import java.util.regex.Pattern;

/**
 * @author fboesiger
 */
public class FileUploadDetector extends Detector {
    @Override
    public void addElements(PDDocument document, Map<ElementType, Integer> elements) throws IOException {
        PDFTextStripper reader = new PDFTextStripper();
        String text = reader.getText(document);
        if (Pattern.compile("(?i)Anhang|Anhänge|Attachment|Upload|Einzureichen").matcher(text).find()) {
            elements.put(ElementType.FILEUPLOAD, 1);
        }
    }
}
